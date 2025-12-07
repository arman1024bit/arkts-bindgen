use std::path::{Path, PathBuf};

/// 计算从 from_decl 所在目录到 target_src 目录的相对路径，
/// 并保证返回值以 `/` 结尾，例如： "../../../ets/pages/"
/// from_decl: ./harmony/entry/src/main/cpp/types/libentry/Index.d.ts
/// target_src: /harmony_null/entry/src/main/ets/pages 或者 带/
pub(crate) fn calc_declaration_import_dir(from_decl: &str, target_src: &str) -> String {
    // from_decl 所在目录
    let from_path = Path::new(from_decl);
    let from_dir = from_path.parent().unwrap_or_else(|| Path::new(""));

    // target_src 目录（有没有结尾的 / 都无所谓，Path 会自动处理）
    let target_dir = Path::new(target_src);

    // 分解为组件，方便找公共前缀
    let from_comps: Vec<_> = from_dir.components().collect();
    let to_comps: Vec<_> = target_dir.components().collect();

    // 找两条路径的最长公共前缀长度 i
    let mut i = 0;
    while i < from_comps.len() && i < to_comps.len() && from_comps[i] == to_comps[i] {
        i += 1;
    }

    // 需要从 from_dir 退回多少级目录：from_comps.len() - i
    let up_count = from_comps.len().saturating_sub(i);

    // 组装相对路径：先加若干个 ".."，再加 target 剩余路径
    let mut rel = PathBuf::new();
    for _ in 0..up_count {
        rel.push("..");
    }
    for comp in &to_comps[i..] {
        rel.push(comp.as_os_str());
    }

    // 转为字符串，统一成 POSIX 风格，并保证结尾有 `/`
    let mut s = rel.to_string_lossy().replace('\\', "/");
    if !s.ends_with('/') {
        s.push('/');
    }
    s
}
