use gpui_kit::assets::Assets;
use gpui_kit::component::{
    ActiveTheme, StyledExt,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonVariants as _},
    input::{Input, InputState, Textarea, TextareaState},
    switch::Switch,
};
use gpui_kit::*;

struct ArabicDemo {
    rtl: bool,
    enabled: bool,
    name: Entity<InputState>,
    notes: Entity<TextareaState>,
}
impl ArabicDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            rtl: true,
            enabled: true,
            name: cx.new(|cx| InputState::new(window, cx).placeholder("اكتب الاسم — Name")),
            notes: cx.new(|cx| {
                TextareaState::new(window, cx)
                    .default_value("أَهْلاً بالعالم!\nGPUI يدعم العربية و English (123).\nجرّب التحديد، الأسهم، الحذف، وتغيير حجم النافذة.")
            }),
        }
    }
}
impl Render for ArabicDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .direction(if self.rtl {
                LayoutDirection::Rtl
            } else {
                LayoutDirection::Ltr
            })
            .size_full()
            .p_6()
            .gap_4()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child("واجهة هُلول — Arabic & bidirectional text")
            .child(
                Breadcrumb::new()
                    .child(BreadcrumbItem::new("الرئيسية"))
                    .child(BreadcrumbItem::new("الإعدادات")),
            )
            .child(
                Button::new("direction")
                    .primary()
                    .label(if self.rtl {
                        "Switch to LTR"
                    } else {
                        "التحويل إلى RTL"
                    })
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.rtl = !this.rtl;
                        cx.notify();
                    })),
            )
            .child(Input::new(&self.name))
            .child(Textarea::new(&self.notes).h(px(180.)))
            .child(
                Switch::new("enabled")
                    .label("تفعيل الإشعارات")
                    .checked(self.enabled)
                    .on_change(cx.listener(|this, checked: &bool, _, cx| {
                        this.enabled = *checked;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .ltr()
                    .ps(px(12.))
                    .child("LTR island: let greeting = \"مرحبا\";"),
            )
    }
}
fn main() {
    gpui_kit::application().with_assets(Assets).run(|cx| {
        gpui_kit::init(cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(760.), px(560.)), cx)),
            ..Default::default()
        };
        gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| ArabicDemo::new(window, cx))
        })
        .expect("open Arabic RTL example");
    });
}
