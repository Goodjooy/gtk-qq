use std::sync::Arc;

use relm4::{
    adw::HeaderBar,
    gtk::{
        self,
        traits::{BoxExt, ButtonExt, GtkWindowExt},
        Button, Label,
    },
    Component, ComponentController, ComponentSender, WidgetPlus,
};

use crate::link_copier::{self, LinkCopier, LinkCopierModel};

use super::payloads::{self, Output};

pub struct Widgets {
    _header_bar: HeaderBar,
    _body: gtk::Box,

    _msg: Label,
    _link: LinkCopier,
    _msg2: Label,
    _btn: Button,
}

impl Widgets {
    pub(super) fn new(
        root: &gtk::Box,
        cfg: payloads::Payload,
        sender_ref: &ComponentSender<super::DeviceLock>,
    ) -> Self {
        let header_bar = HeaderBar::builder()
            .title_widget(&Label::new("Device Lock Verify Introduction".into()))
            .build();

        let body = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .vexpand(true)
            .spacing(24)
            .build();

        body.set_margin_all(16);

        let msg = Label::new(
            format!(
                "Please open the link below and use your logged in device[sms:{}] to verify",
                cfg.sms_phone.unwrap_or("<unknown>".into())
            )
            .as_str()
            .into(),
        );

        

        let link = LinkCopierModel::builder()
            .launch(
                link_copier::Payload::builder()
                    .url(cfg.unlock_url)
                    .label("Device Lock Verification".into())
                    .build(),
            )
            .forward(sender_ref.output_sender(), |msg| match msg {
                link_copier::Output::LinkCopied => payloads::Output::CopyLink,
            });

        let msg2 = Label::new("Once verified, click the button below".into());

        let btn = Button::builder()
            .label("Confirm Verification")
            .build();

        let sender = Arc::clone(sender_ref);
        btn.connect_clicked(move |_| {
            sender.output(Output::ConfirmVerify);
            cfg.window.close();
        });

        root.append(&header_bar);
        root.append(&body);

        body.append(&msg);
        body.append(link.widget());
        body.append(&msg2);
        body.append(&btn);

        Widgets {
            _header_bar: header_bar,
            _body: body,
            _msg: msg,
            _link: link,
            _msg2: msg2,
            _btn: btn,
        }
    }
}
