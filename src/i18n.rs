//! Minimal i18n for the gallery app. Supports EN and JA.

#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Ja,
}

impl Lang {
    pub fn from_args() -> Self {
        if std::env::args().any(|a| a == "--ja" || a == "-j") {
            Lang::Ja
        } else {
            Lang::En
        }
    }
}

/// Localized strings for the gallery.
pub struct L {
    pub lang: Lang,
}

impl L {
    pub fn new(lang: Lang) -> Self { Self { lang } }

    // Tab labels (short)
    pub fn tab_basic(&self) -> &str { match self.lang { Lang::En => "Basic", Lang::Ja => "基本" } }
    pub fn tab_input(&self) -> &str { match self.lang { Lang::En => "Input", Lang::Ja => "入力" } }
    pub fn tab_layout(&self) -> &str { match self.lang { Lang::En => "Layout", Lang::Ja => "配置" } }
    pub fn tab_nav(&self) -> &str { match self.lang { Lang::En => "Nav", Lang::Ja => "案内" } }
    pub fn tab_display(&self) -> &str { match self.lang { Lang::En => "Display", Lang::Ja => "表示" } }
    pub fn tab_overlay(&self) -> &str { match self.lang { Lang::En => "Overlay", Lang::Ja => "重畳" } }
    pub fn tab_new(&self) -> &str { match self.lang { Lang::En => "New", Lang::Ja => "新規" } }

    // Basic page
    pub fn button(&self) -> &str { match self.lang { Lang::En => "Button", Lang::Ja => "ボタン" } }
    pub fn checkbox(&self) -> &str { match self.lang { Lang::En => "Checkbox", Lang::Ja => "チェックボックス" } }
    pub fn switch(&self) -> &str { match self.lang { Lang::En => "Switch", Lang::Ja => "スイッチ" } }
    pub fn text(&self) -> &str { match self.lang { Lang::En => "Text", Lang::Ja => "テキスト" } }
    pub fn default_button(&self) -> &str { match self.lang { Lang::En => "Default", Lang::Ja => "デフォルト" } }
    pub fn action(&self) -> &str { match self.lang { Lang::En => "Action", Lang::Ja => "実行" } }
    pub fn disabled(&self) -> &str { match self.lang { Lang::En => "Disabled", Lang::Ja => "無効" } }
    pub fn option_a(&self) -> &str { match self.lang { Lang::En => "Option A", Lang::Ja => "選択肢A" } }
    pub fn option_b_checked(&self) -> &str { match self.lang { Lang::En => "Option B (on)", Lang::Ja => "選択肢B (オン)" } }
    pub fn option_c(&self) -> &str { match self.lang { Lang::En => "Option C", Lang::Ja => "選択肢C" } }
    pub fn dark_mode(&self) -> &str { match self.lang { Lang::En => "Dark Mode", Lang::Ja => "ダークモード" } }
    pub fn notifications(&self) -> &str { match self.lang { Lang::En => "Notifications", Lang::Ja => "通知" } }
    pub fn text_sample(&self) -> &str { match self.lang { Lang::En => "This is a TextWidget with default styling.", Lang::Ja => "TextWidgetのデフォルトスタイル表示です。" } }
    pub fn larger_text(&self) -> &str { match self.lang { Lang::En => "Larger text (18px)", Lang::Ja => "大きめテキスト (18px)" } }

    // Input page
    pub fn text_input(&self) -> &str { match self.lang { Lang::En => "TextInput", Lang::Ja => "テキスト入力" } }
    pub fn slider(&self) -> &str { match self.lang { Lang::En => "Slider", Lang::Ja => "スライダー" } }
    pub fn spin_button(&self) -> &str { match self.lang { Lang::En => "SpinButton", Lang::Ja => "スピンボタン" } }
    pub fn dropdown(&self) -> &str { match self.lang { Lang::En => "Dropdown", Lang::Ja => "ドロップダウン" } }
    pub fn color_picker(&self) -> &str { match self.lang { Lang::En => "ColorPicker", Lang::Ja => "色選択" } }
    pub fn type_here(&self) -> &str { match self.lang { Lang::En => "Type here...", Lang::Ja => "ここに入力..." } }
    pub fn continuous(&self) -> &str { match self.lang { Lang::En => "Continuous (0-100)", Lang::Ja => "連続値 (0-100)" } }
    pub fn stepped(&self) -> &str { match self.lang { Lang::En => "Stepped (0.0-1.0)", Lang::Ja => "段階値 (0.0-1.0)" } }
    pub fn integer(&self) -> &str { match self.lang { Lang::En => "Integer", Lang::Ja => "整数" } }
    pub fn float_val(&self) -> &str { match self.lang { Lang::En => "Float", Lang::Ja => "小数" } }
    pub fn city(&self) -> &str { match self.lang { Lang::En => "City:", Lang::Ja => "都市:" } }

    // Layout page
    pub fn hstack_vstack(&self) -> &str { match self.lang { Lang::En => "HStack / VStack", Lang::Ja => "横並び / 縦並び" } }
    pub fn grid(&self) -> &str { match self.lang { Lang::En => "GridLayout (3 cols)", Lang::Ja => "グリッド (3列)" } }
    pub fn split_view(&self) -> &str { match self.lang { Lang::En => "SplitView (drag)", Lang::Ja => "分割ビュー (ドラッグ)" } }
    pub fn left_pane(&self) -> &str { match self.lang { Lang::En => "Left pane", Lang::Ja => "左ペイン" } }
    pub fn right_pane(&self) -> &str { match self.lang { Lang::En => "Right pane", Lang::Ja => "右ペイン" } }

    // Navigation page
    pub fn nav_widgets(&self) -> &str { match self.lang { Lang::En => "Navigation Widgets", Lang::Ja => "ナビゲーション" } }
    pub fn tab_view_nested(&self) -> &str { match self.lang { Lang::En => "TabView (nested)", Lang::Ja => "タブビュー (入れ子)" } }
    pub fn tree_view(&self) -> &str { match self.lang { Lang::En => "TreeView", Lang::Ja => "ツリービュー" } }

    // Display page
    pub fn progress_bar(&self) -> &str { match self.lang { Lang::En => "ProgressBar", Lang::Ja => "プログレスバー" } }
    pub fn image_widget(&self) -> &str { match self.lang { Lang::En => "ImageWidget", Lang::Ja => "画像ウィジェット" } }
    pub fn status_bar(&self) -> &str { match self.lang { Lang::En => "StatusBar", Lang::Ja => "ステータスバー" } }
    pub fn indeterminate(&self) -> &str { match self.lang { Lang::En => "Indeterminate", Lang::Ja => "不定" } }
    pub fn checkerboard_desc(&self) -> &str { match self.lang { Lang::En => "128x128 checkerboard (generated RGBA)", Lang::Ja => "checkerboard 128x128" } }

    // Overlay page
    pub fn overlay_widgets(&self) -> &str { match self.lang { Lang::En => "Overlay Widgets", Lang::Ja => "オーバーレイ" } }
    pub fn toast(&self) -> &str { match self.lang { Lang::En => "Toast", Lang::Ja => "トースト" } }
    pub fn welcome(&self) -> &str { match self.lang { Lang::En => "Welcome to Hayate Gallery!", Lang::Ja => "Hayate ギャラリーへようこそ！" } }

    // New widgets page
    pub fn new_widgets(&self) -> &str { match self.lang { Lang::En => "New Widgets (7)", Lang::Ja => "新規ウィジェット (7)" } }
    pub fn text_area(&self) -> &str { match self.lang { Lang::En => "TextArea", Lang::Ja => "テキストエリア" } }
    pub fn list_view(&self) -> &str { match self.lang { Lang::En => "ListView (50 items)", Lang::Ja => "リストビュー (50件)" } }
    pub fn breadcrumb(&self) -> &str { match self.lang { Lang::En => "Breadcrumb", Lang::Ja => "パンくず" } }
    pub fn radio_group(&self) -> &str { match self.lang { Lang::En => "RadioGroup", Lang::Ja => "ラジオボタン" } }
    pub fn canvas_view(&self) -> &str { match self.lang { Lang::En => "CanvasView (custom)", Lang::Ja => "キャンバス (カスタム描画)" } }
    pub fn nav_list(&self) -> &str { match self.lang { Lang::En => "NavigationList", Lang::Ja => "ナビリスト" } }
    pub fn thumb_list(&self) -> &str { match self.lang { Lang::En => "ThumbnailList", Lang::Ja => "サムネイル一覧" } }
    pub fn textarea_placeholder(&self) -> &str { match self.lang { Lang::En => "Multi-line input...", Lang::Ja => "複数行入力..." } }
}
