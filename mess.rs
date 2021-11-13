////////////////////////////////////////////////////////////////////////////////
// Adding new messages

impl MessagingUI {
    fn add_timestamp(&mut self, ts: Timestamp) {
        if let Some(ts_) = self.last_activity_ts {
            if ts_ != ts {
                ts.stamp(&mut self.msg_area);
            } else if matches!(self.msg_area.layout(), Layout::Aligned { .. }) {
                Timestamp::blank(&mut self.msg_area)
            }
        } else {
            ts.stamp(&mut self.msg_area);
        }
        self.last_activity_ts = Some(ts);
    }

    pub(crate) fn show_topic(&mut self, topic: &str, ts: Timestamp) {
        self.add_timestamp(ts);

        self.msg_area.add_text(topic, SegStyle::Topic);

        self.msg_area.flush_line();
    }

    pub(crate) fn add_client_err_msg(&mut self, msg: &str) {
        self.reset_activity_line();

        self.msg_area.add_text(msg, SegStyle::ErrMsg);
        self.msg_area.flush_line();
    }

    pub(crate) fn add_client_notify_msg(&mut self, msg: &str) {
        self.reset_activity_line();

        self.msg_area.add_text(msg, SegStyle::Faded);
        self.msg_area.flush_line();
        self.reset_activity_line();
    }
}
