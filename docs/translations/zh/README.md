# 2048

逐步教程，引导读者使用 Bevy 引擎实现 2048 项目。

## 步骤

基本目标是 2048 经典版（classic），分为四个步骤。

| 步骤                                     | 源码                                 | 说明                                       | 特性                          |
| ---------------------------------------- | ------------------------------------ | ------------------------------------------ | ----------------------------- |
| [步骤零：配置项目](./step0_setup.md)     | [step0_setup](../../../step0_setup/) | 配置项目（依赖项、开发体验）               | `Commands`,`Camera2d`, `Text` |
| [步骤一：棋盘](./step1_board.md)         | [step1_board](../../../step1_board/) | 简陋棋盘可视化（二维图形、）               | `Mesh2d`                      |
| [步骤二：逻辑](./step2_logic.md)         | [step2_logic](../../../step2_logic/) | 基本程序逻辑（合并、计分、生成、结束条件） | `ButtonInput`                 |
| [步骤三：美化](./step3_beautify.md)      | TODO                                 | 美化界面                                   |                               |
| [步骤迄：发布](./stepA_distribute.md)    | TODO                                 | 发布（桌面、网页、手机、本地化）           |                               |
| [拓展一：标准版](./stepX1_standard.md)   | TODO                                 | 「标准版」功能                             |                               |
| [拓展二：图像](./stepX2_image.md)        | TODO                                 | 图像                                       |                               |
| [拓展三：特殊合并规则](./stepX3_rule.md) | TODO                                 | 特殊合并规则                               |                               |
| [拓展四：三维](./stepX4_3d.md)           | TODO                                 | 三维                                       |                               |
| [拓展五：排名](./stepX5_rank.md)         | TODO                                 | 使用远程服务器记录和展示排名               |                               |
| [拓展六：对战](./stepX6_versus.md)       | TODO                                 | 对战（服务器或者 RTC）                     |                               |
