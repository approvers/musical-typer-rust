# Musical Typer Rust

Musical Typer を Rust で実装チャレンジ!!!


# 仕様

楽曲が流れながら、歌詞とそのローマ字が画面に表示されます。

制限時間内に素早くタイピングして、譜面をクリアしましょう。


## イベント

画面=>ゲーム:
- キー入力
- 時間の経過

ゲーム=>画面:
- BGM の再生/停止
- 残り時間の減少
- 入力する文字列の更新
- タイプミス


## 画面

- タイトル
- 作者
- 今のセクション
  - 残り時間
  - 入力する歌詞
    - すでに入力した文字列
    - これから入力する文字列
  - 入力するローマ字
    - すでに入力した文字列
    - これから入力する文字列
- 仮想キーボード


## 譜面

- メタデータ
  - 曲名
  - 作者
  - 歌手
  - 譜面作者
  - 楽曲ファイルパス
- コマンド
  - BPM
  - セクション
    - 時間
    - 分(省略可)
    - 歌詞
    - 歌詞の読み仮名


# ディレクトリ構造

- asset -- 外部ファイル
- example -- 譜面データのサンプル
- src -- Rust ソースコード
  - abst -- 入出力などの抽象
  - exp -- システムに出てくる概念
  - op -- システムで重要な操作
  - skin -- abst の実装
    - sdl_presenter -- Sdl2 画面のコンポーネント


# 譜面フォーマット

ファイルから読み込む譜面フォーマットの文法です。


## コメント

`#` から改行までがコメントです。

```
# これはコメント
```


## プロパティ

`:プロパティ名 値` がプロパティです。

プロパティには、以下のものがあります。

| プロパティ名    | 指定する値       |
| --------------- | ---------------- |
| `title`         | 曲名             |
| `song_author`   | 作曲者           |
| `lyrics_author` | 作詞者           |
| `singer`        | 歌手             |
| `score_author`  | 譜面作成者       |
| `song_data`     | 曲の音声ファイル |
| `bpm`           | BPM              |


## コマンド

`[コマンド名]` と改行がコマンドです。

`[start]`、`[break]`、`[end]` のコマンドがあります。

`[start]` から `[end]` までが歌詞定義です。

```
[start]
# 歌詞定義
[end]
```

`[break]` は非推奨です。効果を持ちません。


## 歌詞定義

ここで、歌詞とその出現タイミングを定義します。


### 時間指定

時間指定の構文で、歌詞の文が切り替わるタイミングを指定します。

`*`	の後に小数で秒を指定します。

```
# 27.750 秒
*27.750
```

`|` の後に整数で分を指定します。

```
# 3 分
|3
```

`@`	の後に文でセクションを指定します。

```
@Aメロ
```


### 文指定

前の時間指定から次の時間指定文指定まで、定義した文が出現します。

`:` の後に直前の歌詞の読み仮名を書きます。スペースなどを入れずにひらがなだけにしてください。

この読み仮名を、前の時間指定から次の時間指定文指定までの間にタイピングします。

```
*55.250
千本桜　夜ニ紛レ
:せんぼんざくらよるにまぎれ

*59.500
```

`>>` の後にキャプションなどを書くことができます。これは表示されますがタイピングにはなりません。読み仮名も定義できません。

```
*0.0
>> (前奏)

*11.5
```

何も定義しなかった場合は何も表示されません。キャプションが必要ない場合はそうしてください。
