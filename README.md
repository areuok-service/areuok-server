# areuok-server - 云端服务端

[English](./README.en.md) | 简体中文

AreUOK 云端服务端，为设备管理和签到跟踪系统提供高性能后端服务，基于 Rust 和 PostgreSQL 构建。

> 🎯 **areuok-server** 是 areuok 客户端应用的配套云端服务，支持设备管理、多设备监督、签到数据同步和连续签到统计等功能。

## 功能特性

- 📱 **设备管理** - 注册和管理多种模式的设备
- 🔐 **IMEI 绑定** - 可选的设备 IMEI 绑定，支持设备恢复和身份识别
- 🏷️ **设备名称管理** - 全局唯一设备名称，15 天更新冷却期
- 🔥 **签到跟踪** - 追踪每日签到，自动计算连续签到天数
- 👀 **监督系统** - 创建设备间的监督关系
- 🌐 **RESTful API** - 清晰直观的 HTTP API
- 🗄️ **数据库迁移** - 自动化模式管理
- 🐳 **Docker 支持** - 容器化部署，轻松快速设置
- ✅ **全面测试** - 使用 pytest 提供完整的测试覆盖

## 项目架构

本项目采用 Cargo 工作区结构：

```
┌─────────────────────────────────────────────────────────────┐
│                    areuok Server                           │
│              (Rust + Axum + PostgreSQL)                     │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   models    │    │     db      │    │    api      │     │
│  │  数据结构   │    │ 数据库连接  │    │  HTTP 路由   │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐   │
│  │                   server                             │   │
│  │                 应用入口和配置                         │   │
│  └───────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
                   PostgreSQL 数据库
```

### Crate 职责

- **models** - 数据结构（Device, SupervisionRequest, SigninRecord）
- **db** - 数据库连接池、迁移和查询助手
- **api** - HTTP 处理器、路由和错误处理
- **server** - 应用引导、配置和主入口点

## 快速开始

### 前置要求

- Docker 和 Docker Compose
- 或者 Rust 1.70+ 和 PostgreSQL 12+（用于本地开发）

### 最快启动方式

```bash
# 克隆仓库并一键启动
git clone https://github.com/nicepeng/areuok-server.git
cd areuok-server
./start-docker.sh
```

服务将在 `http://localhost:3000` 启动

### 快速测试

```bash
# 测试设备注册
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{"device_name": "my-device", "mode": "signin"}'

# 运行所有 API 测试
cd test
uv run pytest -v
```

## 安装部署

### Docker 部署（推荐）

#### 方式 1：使用启动脚本

```bash
./start-docker.sh
```

此脚本会：
1. 构建 Docker 镜像
2. 启动 PostgreSQL 和服务端容器
3. 显示日志

#### 方式 2：使用 Makefile

```bash
# 启动所有服务（需要时构建 Docker 镜像）
make up

# 从头开始构建并启动
make run

# 查看日志
make logs

# 仅查看服务端日志
make logs-server

# 停止服务
make down

# 停止并删除所有数据（包括数据库）
make clean

# 仅构建 Docker 镜像
make build

# 运行测试
make test
```

#### 方式 3：手动 Docker 命令

```bash
# 构建 Docker 镜像
docker build -t areuok-server .

# 启动所有服务（PostgreSQL + 服务端）
docker-compose up -d

# 查看日志
docker-compose logs -f server

# 停止服务
docker-compose down

# 停止并删除卷（清除数据库数据）
docker-compose down -v

# 重启服务
docker-compose restart server
```

#### Docker 服务

- **PostgreSQL**: `localhost:5432`
  - 用户名: `postgres`
  - 密码: `postgres`
  - 数据库: `areuok`
- **Server**: `localhost:3000`

### 本地开发部署

#### 1. 安装依赖

```bash
# 克隆仓库
git clone https://github.com/nicepeng/areuok-server.git
cd areuok-server

# 安装 Rust 依赖
cargo build
```

#### 2. 配置数据库

确保已安装 PostgreSQL 12+ 并创建数据库：

```bash
# 创建数据库
createdb areuok

# 配置环境变量
cp .env.example .env
# 编辑 .env 文件，设置 DATABASE_URL
```

#### 3. 运行迁移

```bash
# 在运行服务器之前运行迁移
make migrate

# 或者使用 cargo
cargo run -- migrate
```

#### 4. 启动服务器

```bash
# 开发模式
cargo run

# 或者使用 make
make run
```

服务器将在 `http://localhost:3000` 启动

### 数据库迁移

```bash
# 运行迁移（需要运行服务器）
make migrate

# 在 Docker 中运行迁移
make migrate-docker
```

## API 文档

### 设备管理

| 端点 | 方法 | 描述 |
|------|------|------|
| `/devices/register` | POST | 注册新设备 |
| `/devices/{id}` | GET | 获取设备信息 |
| `/search/devices?q={query}` | GET | 搜索设备（最少2个字符） |
| `/devices/{id}/signin` | POST | 设备签到 |
| `/devices/{id}/status` | GET | 获取签到状态 |

### 设备注册

```bash
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "my-device",
    "mode": "signin",
    "imei": "optional-imei"
  }'
```

响应：
```json
{
  "device_id": "uuid",
  "device_name": "my-device",
  "mode": "signin",
  "imei": "optional-imei",
  "created_at": "2025-01-15T10:00:00Z"
}
```

### 设备签到

```bash
curl -X POST http://localhost:3000/devices/{device_id}/signin
```

响应：
```json
{
  "device_id": "uuid",
  "signin_date": "2025-01-15",
  "streak": 5,
  "created_at": "2025-01-15T10:00:00Z"
}
```

### 获取签到状态

```bash
curl http://localhost:3000/devices/{device_id}/status
```

响应：
```json
{
  "device_id": "uuid",
  "device_name": "my-device",
  "mode": "signin",
  "current_streak": 5,
  "last_signin_date": "2025-01-15",
  "today_signed_in": true
}
```

### 监督管理

| 端点 | 方法 | 描述 |
|------|------|------|
| `/supervision/request` | POST | 发起监督请求 |
| `/supervision/pending/{id}` | GET | 获取待处理的监督请求 |
| `/supervision/accept` | POST | 接受监督请求 |
| `/supervision/reject` | POST | 拒绝监督请求 |
| `/supervision/list/{id}` | GET | 获取监督关系列表 |
| `/supervision/{relation_id}` | DELETE | 删除监督关系 |

### 发起监督请求

```bash
curl -X POST http://localhost:3000/supervision/request \
  -H "Content-Type: application/json" \
  -d '{
    "requester_id": "device-uuid-1",
    "target_id": "device-uuid-2"
  }'
```

### 接受监督请求

```bash
curl -X POST http://localhost:3000/supervision/accept \
  -H "Content-Type: application/json" \
  -d '{
    "request_id": "request-uuid"
  }'
```

### 获取监督关系列表

```bash
curl http://localhost:3000/supervision/list/{device_id}
```

响应：
```json
{
  "supervising": [
    {
      "relation_id": "uuid",
      "target_device_name": "device-name",
      "target_device_id": "uuid",
      "created_at": "2025-01-15T10:00:00Z"
    }
  ],
  "supervised_by": [
    {
      "relation_id": "uuid",
      "supervisor_device_name": "supervisor-name",
      "supervisor_device_id": "uuid",
      "created_at": "2025-01-15T10:00:00Z"
    }
  ]
}
```

## 测试

### Rust 测试

```bash
# 运行所有 Rust 测试
cargo test

# 使用 make
make test
```

### 集成测试

```bash
# 使用 Docker Compose 运行集成测试
make test-integration
```

### API 测试

```bash
cd test

# 安装 Python 依赖
uv pip install -r requirements.txt

# 运行所有测试
uv run pytest -v

# 运行特定测试
uv run pytest -v -k "test_device"

# 查看测试覆盖率
uv run pytest --cov=. --cov-report=html
```

## 开发

### 环境要求

- Rust 1.70+
- PostgreSQL 12+
- Docker 和 Docker Compose（推荐）

### 代码检查

```bash
# 快速检查编译
cargo check

# 构建项目
cargo build

# 运行服务器
cargo run
```

### 代码质量

```bash
# 格式化代码
make fmt

# 运行 Clippy linter
make clippy

# 或者使用 cargo
cargo fmt
cargo clippy -- -D warnings
```

### 数据库操作

```bash
# 运行迁移
make migrate

# 查看数据库模式
cat crates/db/migrations/*.sql
```

## 环境变量

| 变量 | 必需 | 描述 | 默认值 |
|------|------|------|--------|
| `DATABASE_URL` | 是 | PostgreSQL 连接字符串 | - |
| `RUST_LOG` | 否 | 日志级别 | `info,server=debug,api=debug,db=debug` |

### 环境变量配置示例

```bash
# .env 文件示例
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/areuok
RUST_LOG=info,server=debug,api=debug,db=debug
```

## 技术栈

- **语言**: Rust 1.70+
- **Web 框架**: Axum 0.7
- **数据库**: PostgreSQL 16
- **数据库驱动**: SQLx 0.8
- **异步运行时**: Tokio 1.49
- **序列化**: Serde 1.0
- **容器化**: Docker & Docker Compose
- **测试**: pytest (Python API 测试)

## 数据库模式

数据库模式位于 `crates/db/migrations/` 目录，包含以下表：

- `devices` - 设备信息
- `signin_records` - 签到记录
- `supervision_requests` - 监督请求
- `supervision_relations` - 监督关系

详细的数据库模式文档请参阅 [docs/database-schema.md](docs/database-schema.md)

## 项目结构

```
areuok-server/
├── Cargo.toml              # 工作区配置
├── Dockerfile              # Docker 镜像定义
├── docker-compose.yml      # Docker 服务编排
├── start-docker.sh        # 快速启动脚本
├── Makefile               # 常用命令
├── .env.example           # 环境变量示例
├── crates/
│   ├── models/            # 数据结构和类型
│   ├── db/                # 数据库连接和迁移
│   ├── api/               # HTTP API 处理器和路由
│   └── server/            # 主入口点和配置
├── test/                  # API 测试（Python/pytest）
│   ├── conftest.py       # 测试配置
│   ├── test_device.py    # 设备相关测试
│   ├── test_signin.py    # 签到相关测试
│   └── test_supervision.py # 监督相关测试
└── docs/                  # 额外文档
    ├── database-schema.md
    └── lint-setup.md
```

## 部署

### Docker 部署

生产环境部署建议使用 Docker Compose：

```bash
# 构建生产镜像
docker build -t areuok-server:latest .

# 使用生产配置启动
docker-compose -f docker-compose.yml up -d

# 查看日志
docker-compose logs -f
```

### 生产环境注意事项

1. **数据库安全**
   - 修改默认的 PostgreSQL 密码
   - 使用强密码
   - 限制数据库访问 IP

2. **HTTPS 配置**
   - 使用反向代理（如 Nginx）
   - 配置 SSL 证书

3. **日志管理**
   - 配置适当的日志级别
   - 设置日志轮转

4. **监控**
   - 添加健康检查端点
   - 监控服务器性能

## 故障排除

### 常见问题

**1. 数据库连接失败**

```bash
# 检查 PostgreSQL 是否运行
docker ps | grep postgres

# 检查数据库连接字符串
echo $DATABASE_URL

# 重启数据库
docker-compose restart postgres
```

**2. 端口已被占用**

```bash
# 检查端口占用
lsof -i :3000
lsof -i :5432

# 修改 docker-compose.yml 中的端口映射
```

**3. 迁移失败**

```bash
# 检查迁移文件
ls -la crates/db/migrations/

# 查看迁移日志
docker-compose logs postgres

# 重新运行迁移
docker-compose down -v
docker-compose up -d
```

**4. 测试失败**

```bash
# 确保服务正在运行
curl http://localhost:3000

# 查看服务器日志
docker-compose logs server

# 清理并重新启动
docker-compose down
docker-compose up -d
```

## IDE 推荐

[VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

## 文档

- [数据库模式文档](docs/database-schema.md)
- [代码规范设置](docs/lint-setup.md)
- [贡献指南](CONTRIBUTING.md)
- [许可证](LICENSE)

## 相关仓库

| 仓库 | 描述 |
|------|------|
| [areuok](https://github.com/nicepeng/areuok) | 📱 客户端应用（Tauri + SvelteKit） |
| [areuok-server](https://github.com/nicepeng/areuok-server) | ☁️ 云端服务端（本仓库） |

## 许可证

本项目采用 [GNU General Public License v2.0 (GPLv2)](./LICENSE) 开源协议。

## 致谢

### 技术栈致谢

本项目的实现离不开以下优秀的开源项目：

**核心框架**
- [Rust](https://www.rust-lang.org/) - 安全、并发、高性能的系统编程语言
- [Axum](https://github.com/tokio-rs/axum) - 模块化和易于使用的 Web 框架
- [Tokio](https://tokio.rs/) - Rust 的异步运行时

**数据库**
- [PostgreSQL](https://www.postgresql.org/) - 强大的开源关系数据库
- [SQLx](https://github.com/launchbadge/sqlx) - 异步、纯 Rust 的 SQL 工具包

**工具库**
- [Serde](https://serde.rs/) - Rust 序列化/反序列化框架
- [Chrono](https://github.com/chronotope/chrono) - Rust 日期时间库
- [UUID](https://github.com/uuid-rs/uuid) - UUID 生成和解析

**DevOps**
- [Docker](https://www.docker.com/) - 容器化平台
- [Docker Compose](https://docs.docker.com/compose/) - 多容器 Docker 应用程序

感谢所有开源贡献者的辛勤付出！

## 贡献

欢迎提交 Issue 和 Pull Request！请先阅读 [贡献指南](./CONTRIBUTING.md)。
