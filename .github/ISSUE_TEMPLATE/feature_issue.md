---
name: 业务特性规范
about: 业务功能特性与端到端用例全貌
title: "{特性名称}"
---

## 问题陈述

{从业务与用户视角说明当前面临的问题与痛点}

## 解决方案

{从业务视角说明期望达成的目标与方案}

## 用户故事

1. 作为 {角色}，我想要 {功能}，以便于 {价值}

## 契约与架构决策

- 涉及上下文:
  - `crates/{context}`
- 接口契约:
  - `contracts/{context}/v1/{service}.proto`
- 事件契约:
  - `contracts/events/{event}.v1.json`
- 状态机与不变量:
  - {核心状态流转与不可变约束}

## 测试与验收规范

- 验收特性文件:
  - `crates/{context}/tests/features/{feature_name}.feature`
- 测试接缝与替身:
  - {最高测试接缝与跨模块测试替身约定}

## 范围外

{明确列出本特性不包含、留待后续处理的内容}
