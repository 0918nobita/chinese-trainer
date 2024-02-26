---@meta

---@class LoadStoreOptions
--- `.avro` ファイルのパス
---@field path string
--- `.avro` ファイル内のスキーマの構造から導出されるフィンガープリント
---@field fingerprint string

--- 単語・成績データ等を格納するストア
---@class Store

--- 問題の内容をまとめたデータ
---@class QuizBundle

--- クイズバンドルを出題するスケジュール
---@class QuizBundleSchedule

---@class QuizBundleScheduleConstants
--- 即時出題する
---@field immediate QuizBundleSchedule

--- Flashcard API
---@class FC
--- ストアを読み込む
---@field load_store fun(options: LoadStoreOptions): Store
--- クイズバンドルを作成する
---@field create_quiz_bundle fun(store: Store): QuizBundle
--- クイズバンドルを出題するスケジュール
---@field schedule QuizBundleScheduleConstants
--- クイズバンドルをスケジュールする
---@field schedule_quiz_bundle fun(quiz_bundle: QuizBundle, schedule: QuizBundleSchedule)
fc = {}
