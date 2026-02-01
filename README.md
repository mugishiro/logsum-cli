# logsum-cli

アクセスログを集計する小さなCLIツール。

## 目的
ログファイルを `status` または `path` で集計する。

ログ形式の例:
```
[2026-01-31] GET /index.html 200
[2026-01-31] POST /login 302
```

## 想定CLI
```
logsum <path> --by status
logsum <path> --by path
```

## 仕様（最小）
- 入力1行の形式: `[YYYY-MM-DD] METHOD PATH STATUS`
- 集計対象: `--by status` または `--by path`
- 出力形式: `KEY COUNT`（例: `200 15`, `/login 3`）
- 不正行の扱い: 基本はスキップ（後で厳密化する）

## 1週間プラン（1日30分）

Day 1
- A (20分): 仕様決め + サンプルログ作成 + `cargo run` 骨組み確認
- C (10分): 文字列分割の基礎（`split`, `split_whitespace`）

Day 2
- A (20分): 1行パーサを作る（status + path）
- C (10分): `Option` / `Result` の小問題

Day 3
- A (20分): `HashMap` で集計（status or path）
- C (10分): `HashMap` の追加・更新・走査の練習

Day 4
- A (20分): CLI引数の解析（`--by status|path`）
- C (10分): `std::env::args` のミニ練習

Day 5
- A (20分): 件数で降順ソートして表示
- C (10分): `sort_by_key` の練習

Day 6
- A (20分): エラーハンドリング + 不正行のスキップ
- C (10分): `Result` の伝播練習

Day 7
- A (20分): README + 使い方 + サンプルログ
- C (10分): 復習（過去の練習を1つ解き直す）

## メモ
- 関数は小さく、テストしやすく。
- 時間が足りない日は A または C のどちらかだけ進め、残りは翌日に回す。
