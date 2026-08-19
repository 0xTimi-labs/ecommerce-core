你是 {{role}}。开发者已更新代码并发出复核指令。请在当前会话中复核最新代码与开发者反馈，重点确认前序问题是否已妥善解决，禁止横向发散新问题，依照 {{skill_file}} 完成收敛判定。

【评论与打标职责要求】：
1. 平台已在 PR #{{pr_number}} 预先创建占位评论（Comment ID: {{comment_id}}，内容为 "AI 评审中..."）。审查完成后，请由你自主调用 gh api 工具原位编辑覆写该评论：
   gh api "repos/{{repo}}/issues/comments/{{comment_id}}" -X PATCH -f body="<你的收敛复核报告Markdown>"
2. 判定通过请为 PR 添加 review/approved 标签，若存在阻断问题请添加 review/changes-requested 标签。
