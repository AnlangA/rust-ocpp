# OCPP v2.1 字段顺序优化记录

## 任务目标
对比 `src/tests/schema_validation/schemas/v2.1` 中的JSON schema与 `src/v2_1` 中的Rust数据结构。逐个对比，不允许使用脚本自动化对比。每完成一项，更新本文件。不允许并行

- 最重要的：**需要完成所有的文件与结构体的对比**
- 最重要的：**需要修复代码中与文档不一致的地方**
- 最重要的：**description 与 字段注释也需要对比，保持一致。缺少注释的，需要添加**
- 最重要的：**每次修改后运行cargo test,cargo check进行检查修复**
- 最重要的：跟据SON schema中的 description 补全结构体中字段的注释
- 最重要的：补全单元测试。
- 最重要的: 优化sered序列化格式：尽量将字段上的将rename改成结构体上的rename，如果序列化格式不一致，就在字段上rename.例如：
  
/// Authentication method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APNAuthenticationEnumType {
    #[serde(rename = "PAP")]
    PAP,
    #[serde(rename = "CHAP")]
    CHAP,
    #[serde(rename = "NONE")]
    NONE,
    #[serde(rename = "AUTO")]
    AUTO,
}

改为： 

/// Authentication method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum APNAuthenticationEnumType {
    PAP,
    CHAP,
    NONE,
    AUTO,
}

若不能统一rename，则在字段内单独rename

## 任务结束处理

构建git 提交，并git push到远程服务器

## 任务修改记录

### 2026-01-24 Ralph Loop - Iteration 1

#### Serde 优化任务完成 ✅

**优化文件:**
1. `src/v2_1/enumerations/data_enum.rs` - 使用 `#[serde(rename_all = "lowercase")]` 统一 lowercase
2. `src/v2_1/enumerations/islanding_detection.rs` - 使用 `#[serde(rename_all = "PascalCase")]` 统一 PascalCase

**详细分析:**

对所有 11 个带有 individual `#[serde(rename = "...")]` 的枚举文件进行了分析：

1. **无需优化 (已存在 rename_all):**
   - `phase.rs` - 已使用 `UPPERCASE`
   - `mobility_needs_mode.rs` - 已使用 `UPPERCASE`
   - `der_unit.rs` - 已使用 `PascalCase`
   - `energy_transfer_mode.rs` - 已使用 `PascalCase`

2. **无法优化 (legitimate complex naming):**
   - `measurand.rs` - 混合命名 (PascalCase + dots + acronyms)
   - `connector.rs` - 混合前缀系统 (c/s/w/b + 技术名称)
   - `signing_method.rs` - 加密技术命名 (带连字符)
   - `reading_context.rs` - 点分隔名称
   - `reason.rs` - 无一致模式的 PascalCase

3. **已优化:**
   - `data_enum.rs` - 添加 `#[serde(rename_all = "lowercase")]`
   - `islanding_detection.rs` - 添加 `#[serde(rename_all = "PascalCase")]`

**测试结果:**
- ✅ 所有 2451 个测试通过
- ✅ cargo check 通过
- ✅ 序列化格式与 JSON schema 一致

**代码修改:**
- 优化 2 个枚举文件的 serde 配置
- 修复 2 处测试代码中的枚举变体引用 (RoCoF → Rocof)
