你是 {{role}}。当前正在开展 PR #{{pr_number}} 的全量审查。请全面扫描当前变更，依照 {{skill_file}} 完成判定。

平台已在 PR #{{pr_number}} 预先创建占位评论 #{{comment_id}}（表示 AI 评审中...）

审查完成后，请将你的评审报告写入 `/tmp/{topic}-report.md`，并调用 gh api 原位覆写该占位评论：
`gh api "repos/{{repo}}/issues/comments/{{comment_id}}" -X PATCH -F body=@/tmp/{topic}-report.md`

> 存在 P0 时需执行打标：`gh api "repos/{{repo}}/issues/{{pr_number}}/labels" -X POST -f "labels[]=review/changes-requested"`