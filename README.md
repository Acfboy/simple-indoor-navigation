## simple-indoor-navigation

This is a hands - on project of mine to create an indoor navigation app based on a mobile compass. I'm using Tauri for the backend, with Vue + TypeScript for the frontend and Rust for the backend.

I've already implemented the original features, but there are still some issues:
1. Poor Module Division: Some parts have a lot of code in a single function or component, which reduces readability and maintainability.
2. Lack of Documentation and Testing: I didn't get into the habit of writing documentation and tests, focusing only on getting the project done quickly.
3. Inadequate Error Handling: Although it generally works fine, the app crashes if given a map that's mostly correct but has some issues.
4. Low Practicality: In practical testing in Qinling Hall, the navigation is not very effective.
5. **Pretentious:** There's no need to write in English in the README at all.

As a learning exercise, I'll gradually refactor some code to make it more elegant and improve its practicality.

这是我的一个练手项目，实现一个基于手机罗盘的室内导航 APP，使用 tauri 实现，前端 Vue + Typescript，后端 Rust。

原定功能已经实现完成，但是还有以下问题：

1. 模块划分不够好。有几个部分单个函数单个组件写了一大堆东西，可读性可维护性较低。
2. 缺少文档和测试。习惯不够好，没有随手写文档和测试，就想着快点把东西做出来。
3. 没有考虑好错误处理。虽然一般不会出问题，但是若给个大部分对但有点问题的地图直接爆炸。
4. 实用性低。在秦岭堂的实战测试表明，这导航一点也不好用。

作为练习，我会慢慢重构部分代码，使实现更加优雅，并提高实用性。