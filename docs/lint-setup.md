# Lint 设置指南

本项目已配置严格的 Rust 代码风格审查和质量检查工具。

## 配置文件

### 1. rustfmt.toml
代码格式化配置，确保代码风格统一。

主要设置：
- 最大行宽: 100 字符
- 使用 4 空格缩进
- 自动重排序导入
- Unix 风格换行符

### 2. clippy.toml
Clippy linter 配置，启用更严格的代码检查。

主要设置：
- 认知复杂度阈值: 30
- 类型复杂度阈值: 250
- 函数参数最多: 7 个
- 要求 crate 级别的文档

### 3. .gitattributes
Git 文件属性配置，确保跨平台一致性。

主要设置：
- 自动检测文本文件
- 统一使用 LF 换行符
- Rust 文件使用 diff 高亮

## Makefile 命令

```bash
# 运行所有检查（格式 + clippy）
make lint

# 格式化代码
make fmt

# 检查格式（不修改）
make fmt-check

# 运行 clippy
make clippy

# 自动修复 clippy 问题
make clippy-fix

# 安装 pre-commit hooks
make install-hooks

# 手动运行 pre-commit 检查
make pre-commit
```

## Pre-commit Hooks

Pre-commit hook 会在每次提交前自动运行以下检查：

1. ✅ 代码格式检查 (rustfmt)
2. ✅ Clippy lint 检查
3. ✅ 单元测试（如果设置了 DATABASE_URL）

安装：
```bash
make install-hooks
```

如果检查失败，你需要先修复问题才能提交。

## 脚本

### scripts/lint
全面的 lint 检查脚本，包括：
- 代码格式检查
- Clippy lint
- 编译检查
- 文档检查
- 尾随空格检查
- Tab 字符检查
- 长行检查

运行：
```bash
./scripts/lint
```

### scripts/pre-commit
Pre-commit hook 脚本，由 Git 自动调用。

## CI/CD 集成

GitHub Actions workflow (`.github/workflows/ci.yml`) 会在每次 push 和 PR 时运行：

1. **Format Check** - 确保代码格式化正确
2. **Clippy Lint** - 捕获代码质量问题
3. **Tests** - 运行所有测试
4. **Build** - 确保项目可以编译

所有检查必须通过才能合并代码。

## 常见问题

### Q: 如何修复格式问题？
```bash
make fmt
```

### Q: 如何修复 Clippy 警告？
```bash
# 查看警告
make clippy

# 自动修复（部分警告）
make clippy-fix
```

### Q: 跳过 pre-commit hook？
```bash
git commit --no-verify -m "message"
```

⚠️ 不推荐！你应该修复代码问题而不是跳过检查。

### Q: 为什么 CI 失败了？
检查 CI 日志，常见原因：
1. 代码未格式化 - 运行 `make fmt`
2. Clippy 警告 - 运行 `make clippy`
3. 测试失败 - 运行 `make test`

## 最佳实践

1. **提交前运行 `make lint`**
   - 确保所有检查通过
   - 减少 CI 循环时间

2. **安装 pre-commit hooks**
   - 自动在提交前检查代码
   - 防止提交有问题的代码

3. **使用 `make clippy-fix`**
   - 自动修复简单的 Clippy 警告
   - 审查自动修复的更改

4. **保持文档更新**
   - 公开 API 需要文档注释
   - 运行 `make doc` 检查文档

5. **编写测试**
   - 新功能需要测试
   - 运行 `make test` 确保测试通过

## 工作流程示例

```bash
# 1. 创建功能分支
git checkout -b feature/new-feature

# 2. 安装 pre-commit hooks（首次）
make install-hooks

# 3. 进行开发
# ... 编写代码 ...

# 4. 运行检查
make lint
make test

# 5. 提交代码（pre-commit hook 自动运行）
git add .
git commit -m "feat: add new feature"

# 6. 如果检查失败，修复并重新提交
make fmt
make clippy-fix
git add .
git commit -m "feat: add new feature"

# 7. 推送并创建 PR
git push origin feature/new-feature
```

## 扩展配置

### 添加新的 Clippy lint
编辑 `clippy.toml`:
```toml
# 你的新配置
```

### 修改代码风格
编辑 `rustfmt.toml`:
```toml
max_width = 120  # 例如：增加行宽
```

运行 `make fmt` 应用更改。

## 参考资源

- [Rustfmt 文档](https://rust-lang.github.io/rustfmt/)
- [Clippy 文档](https://doc.rust-lang.org/clippy/)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
