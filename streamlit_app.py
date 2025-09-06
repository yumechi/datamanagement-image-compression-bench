import io
import os
import re
import sys
from glob import glob
from typing import List, Optional, Tuple

import pandas as pd

try:
    # Use pyarrow engine if available for faster CSV reads
    _CSV_ENGINE = "pyarrow"
    # Probe once to avoid runtime errors on older pandas
    _ = pd.read_csv(io.StringIO("a,b\n1,2\n"), engine=_CSV_ENGINE)
except Exception:
    _CSV_ENGINE = None

import streamlit as st
import plotly.express as px


DEFAULT_DATA_DIR = "result_csv"


def list_csv_candidates(root: str = DEFAULT_DATA_DIR) -> List[str]:
    """List CSVs under the given root directory (recursively).
    Prioritize names that look like format result outputs.
    Returns paths relative to CWD.
    """
    if not root:
        root = "."
    pattern_priority = [
        os.path.join(root, "**", "*format*results*.csv"),
        os.path.join(root, "**", "*format*.csv"),
        os.path.join(root, "**", "*.csv"),
    ]
    seen = set()
    ordered: List[str] = []
    for pat in pattern_priority:
        for p in sorted(glob(pat, recursive=True)):
            if os.path.isfile(p) and p not in seen:
                seen.add(p)
                ordered.append(p)
    return ordered


FORMAT_KEYS = ["jpg", "jpeg", "png", "webp"]


def normalize_cols(df: pd.DataFrame) -> Tuple[pd.DataFrame, dict]:
    mapping = {c: c for c in df.columns}
    lowered = {}
    for c in df.columns:
        lc = c.strip()
        lc = re.sub(r"\s+", "_", lc)
        lc = lc.lower()
        lowered[lc] = c
        mapping[c] = lc
    df2 = df.rename(columns=mapping)
    return df2, lowered


def detect_structure(df: pd.DataFrame) -> Tuple[str, Optional[List[str]]]:
    """
    Return (mode, columns) where mode is:
      - 'long' with a 'format' column present
      - 'wide' with format-specific columns found; columns holds those names
      - 'unknown' if neither detected
    Column names are returned in the normalized (lowercase/underscored) space.
    """
    if "format" in df.columns:
        return "long", None
    # Find columns that look like formats
    format_cols = []
    for c in df.columns:
        for k in FORMAT_KEYS:
            # exact match or contains token like _jpg, -jpg, (jpg)
            if c == k or re.search(rf"(^|[^a-zA-Z]){k}($|[^a-zA-Z])", c):
                format_cols.append(c)
                break
    # De-duplicate and ensure not too many false positives
    format_cols = sorted(set(format_cols))
    if format_cols:
        return "wide", format_cols
    return "unknown", None


def melt_wide(df: pd.DataFrame, value_name: str = "value") -> pd.DataFrame:
    mode, cols = detect_structure(df)
    if mode != "wide" or not cols:
        return df
    id_vars = [c for c in df.columns if c not in cols]
    long_df = df.melt(id_vars=id_vars, value_vars=cols, var_name="format", value_name=value_name)
    # Clean up format values like 'metric_webp' -> 'webp' when obvious
    long_df["format"] = long_df["format"].apply(
        lambda x: next((k.upper() for k in ["jpg", "jpeg", "png", "webp"] if k in str(x).lower()), str(x).upper())
    )
    return long_df


def numeric_columns(df: pd.DataFrame) -> List[str]:
    return [c for c in df.columns if pd.api.types.is_numeric_dtype(df[c])]


def main():
    st.set_page_config(page_title="Image Format Results Viewer", layout="wide")
    st.title("Image Format Results Viewer")
    st.caption("JPG / PNG / WEBP の結果を柔軟に可視化します")

    with st.sidebar:
        st.header("データの選択")
        candidates = list_csv_candidates(DEFAULT_DATA_DIR)
        selected = st.selectbox("CSV ファイル", options=["(ファイルをアップロード)"] + candidates)
        uploaded = st.file_uploader("または CSV をアップロード", type=["csv"]) if selected == "(ファイルをアップロード)" else None
        sep = st.text_input("区切り文字 (自動判定: 空欄)", value="")
        decimal = st.text_input("小数点記号 (通常 '.')", value=".")
        encoding = st.text_input("エンコーディング (自動判定: 空欄)", value="")

    if selected == "(ファイルをアップロード)" and uploaded is None:
        st.info("左のサイドバーで CSV を選択するかアップロードしてください。")
        if candidates:
            st.write("候補 (result_csv/ 配下):")
            st.code("\n".join(candidates))
        else:
            data_root = os.path.abspath(DEFAULT_DATA_DIR)
            st.warning(f"{DEFAULT_DATA_DIR}/ 配下に CSV が見つかりませんでした。データを配置するか、UI からアップロードしてください。\nData dir: {data_root}")
        return

    # Load CSV
    try:
        read_kwargs = {}
        if _CSV_ENGINE:
            read_kwargs["engine"] = _CSV_ENGINE
        if sep:
            read_kwargs["sep"] = sep
        if decimal:
            read_kwargs["decimal"] = decimal
        if encoding:
            read_kwargs["encoding"] = encoding

        if uploaded is not None:
            df_raw = pd.read_csv(uploaded, **read_kwargs)
            source_name = uploaded.name
        else:
            df_raw = pd.read_csv(selected, **read_kwargs)
            source_name = os.path.basename(selected)
    except Exception as e:
        st.error(f"CSV の読み込みに失敗しました: {e}")
        return

    st.subheader("データプレビュー")
    st.caption(source_name)
    st.dataframe(df_raw.head(200), use_container_width=True)

    df_norm, lowered = normalize_cols(df_raw)
    mode, cols = detect_structure(df_norm)

    with st.expander("列の検出結果 / 手動調整", expanded=False):
        st.write(f"検出モード: {mode}")
        if mode == "wide":
            st.write({"format_like_columns": cols})
        st.write("列一覧 (正規化名 → 元名):")
        st.json({k: v for k, v in lowered.items()})

    # Allow manual override for structure
    st.sidebar.header("列の指定")
    manual_mode = st.sidebar.selectbox("データ構造", options=["自動", "long (format 列あり)", "wide (フォーマット列をピボット解除)"])
    use_df = df_norm.copy()
    melted_value_name = "value"

    if manual_mode == "long (format 列あり)":
        if "format" not in use_df.columns:
            # Let user pick a column to treat as format
            fmt_col = st.sidebar.selectbox("format 列", options=list(use_df.columns))
            use_df = use_df.rename(columns={fmt_col: "format"})
    elif manual_mode == "wide (フォーマット列をピボット解除)":
        pre_cols = [c for c in use_df.columns if any(k in c for k in FORMAT_KEYS)]
        selected_cols = st.sidebar.multiselect("フォーマット列 (複数選択)", options=list(use_df.columns), default=pre_cols)
        if selected_cols:
            id_vars = [c for c in use_df.columns if c not in selected_cols]
            use_df = use_df.melt(id_vars=id_vars, value_vars=selected_cols, var_name="format", value_name=melted_value_name)
            use_df["format"] = use_df["format"].apply(
                lambda x: next((k.upper() for k in ["jpg", "jpeg", "png", "webp"] if k in str(x).lower()), str(x).upper())
            )
        else:
            st.warning("ピボット解除する列を選択してください。")
    else:
        # automatic
        if mode == "wide":
            use_df = melt_wide(use_df, value_name=melted_value_name)
        # if unknown and has no 'format', keep as-is and let user choose

    # If we still don't have a 'format' column, attempt to create one from filename (single-file datasets)
    if "format" not in use_df.columns:
        st.info("'format' 列が見つかりません。グラフの色分けは利用できない場合があります。")

    # Column selections
    num_cols = numeric_columns(use_df)
    if not num_cols:
        st.error("数値列が見つかりませんでした。グラフ化できません。")
        return

    # Guess x-axis candidates: quality/crf/bitrate/size/etc.
    x_pref_order = [
        "quality", "q", "crf", "bitrate", "bpp", "filesize", "size", "width", "height", "index",
    ]
    x_default = next((c for c in x_pref_order if c in use_df.columns and c in num_cols), num_cols[0])

    x_col = st.selectbox("X 軸 (数値)", options=num_cols, index=num_cols.index(x_default) if x_default in num_cols else 0)

    if "format" in use_df.columns:
        # Y candidates are other numeric columns, excluding x
        y_candidates = [c for c in num_cols if c != x_col and c != "format"]
        if melted_value_name in use_df.columns and melted_value_name in y_candidates:
            y_default = melted_value_name
        else:
            # Prefer common metrics
            y_pref_order = [
                "psnr", "ssim", "ms-ssim", "lpips", "vmaf", "bits_per_pixel", "bpp", "filesize", "size_kb", "time_ms",
            ]
            y_default = next((c for c in y_pref_order if c in y_candidates), (y_candidates[0] if y_candidates else num_cols[0]))
        y_col = st.selectbox("Y 軸 (数値)", options=y_candidates, index=y_candidates.index(y_default) if y_default in y_candidates else 0)

        # Format filter
        formats = sorted([str(v) for v in use_df["format"].dropna().unique()])
        # Normalize like JPEG->JPG
        def _norm_fmt(x: str) -> str:
            xl = x.lower()
            if "jpeg" in xl or "jpg" in xl:
                return "JPG"
            if "png" in xl:
                return "PNG"
            if "webp" in xl:
                return "WEBP"
            return x.upper()

        use_df["format_norm"] = use_df["format"].astype(str).map(_norm_fmt)
        fmt_options = sorted(use_df["format_norm"].unique().tolist())
        default_fmts = [f for f in ["JPG", "PNG", "WEBP"] if f in fmt_options] or fmt_options
        selected_fmts = st.multiselect("フォーマット フィルタ", options=fmt_options, default=default_fmts)
        plot_df = use_df[use_df["format_norm"].isin(selected_fmts)].copy()
    else:
        # No format column; single series plot
        y_candidates = [c for c in num_cols if c != x_col]
        y_col = st.selectbox("Y 軸 (数値)", options=y_candidates, index=0)
        plot_df = use_df.copy()

    st.subheader("グラフ")
    chart_type = st.radio("チャートタイプ", options=["line", "scatter"], horizontal=True)
    agg = st.checkbox("同一 X 値を平均集計", value=False)
    if agg:
        group_keys = [x_col]
        if "format_norm" in plot_df.columns:
            group_keys.append("format_norm")
        plot_df = plot_df.groupby(group_keys, as_index=False)[y_col].mean()

    if "format_norm" in plot_df.columns:
        if chart_type == "line":
            fig = px.line(plot_df, x=x_col, y=y_col, color="format_norm", markers=True)
        else:
            fig = px.scatter(plot_df, x=x_col, y=y_col, color="format_norm")
    else:
        if chart_type == "line":
            fig = px.line(plot_df, x=x_col, y=y_col, markers=True)
        else:
            fig = px.scatter(plot_df, x=x_col, y=y_col)

    fig.update_layout(legend_title_text="Format")
    st.plotly_chart(fig, use_container_width=True)

    with st.expander("フォーマット別サマリー", expanded=False):
        if "format_norm" in plot_df.columns:
            st.dataframe(plot_df.groupby("format_norm")[y_col].describe(), use_container_width=True)
        else:
            st.dataframe(plot_df[[x_col, y_col]].describe(), use_container_width=True)


if __name__ == "__main__":
    # Allow running as a plain script for quick checks
    if os.environ.get("STREAMLIT_SERVER_ENABLED") == "0":
        # Print basic info for non-UI environments
        csvs = list_csv_candidates(DEFAULT_DATA_DIR)
        print("Found CSVs:")
        for p in csvs:
            print(" -", p)
        sys.exit(0)
    # Otherwise, this will be launched via `streamlit run streamlit_app.py`
    main()
