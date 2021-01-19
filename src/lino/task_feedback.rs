use super::*;

impl Lino {
    pub(crate) fn set_task_feedback_error(&mut self, text: String) {
        self.task_feedback.bg = self.theming.text_frame_bg;
        self.task_feedback.fg = self.theming.error_red;
        self.task_feedback.text = text;
    }

    pub(crate) fn set_task_feedback_normal(&mut self, text: String) {
        self.task_feedback.bg = self.theming.text_frame_bg;
        self.task_feedback.fg = self.theming.text_frame_fg;
        self.task_feedback.text = text;
    }

    pub(crate) fn clear_task_feedback(&mut self) {
        self.task_feedback.bg = self.theming.text_frame_bg;
        self.task_feedback.fg = self.theming.line_nums_frame_fg;
        self.task_feedback.text = "".to_string();
    }
}