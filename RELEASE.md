# 发布指南

## 多平台发布配置

本项目现在支持自动构建和发布到 PyPI，支持以下平台：

### 支持的平台
- **Linux**: x86_64, aarch64
- **Windows**: x64
- **macOS**: x86_64, aarch64

### 发布流程

1. **设置 PyPI API Token**
   - 在 PyPI 上创建 API Token: https://pypi.org/manage/account/token/
   - 在 GitHub 仓库设置中添加 Secret: `PYPI_API_TOKEN`

2. **创建发布版本**
   - 在 GitHub 上创建新的 Release
   - 工作流会自动构建并发布到 PyPI

3. **验证发布**
   - 检查 PyPI 页面: https://pypi.org/project/mdict-rs/
   - 验证所有平台的 wheel 文件都已上传

### 本地测试

```bash
# 构建当前平台的 wheel
./build-wheels.sh

# 安装本地构建的包
pip install dist/*.whl

# 测试功能
python example.py
```

### CI/CD 工作流

- **CI**: 在每次 push/pull request 时运行，构建所有平台的 wheel 文件
- **Publish**: 在创建 GitHub Release 时自动发布到 PyPI

### 依赖要求

- Python >= 3.8
- Rust toolchain
- Maturin (自动安装)