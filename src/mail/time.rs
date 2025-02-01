use chrono::Utc;
use bson::DateTime;

pub trait IntoRelativeTime {
    fn get_created_at(&self) -> DateTime;

    fn into_relative_time(&self) -> String {
        let dt = self.get_created_at().to_chrono();
        let duration = Utc::now().signed_duration_since(dt);

        if duration.num_weeks() > 0 {
            format!("{} week{}", duration.num_weeks(), if duration.num_weeks() > 1 { "s" } else { "" })
        } else if duration.num_days() > 0 {
            format!("{} day{}", duration.num_days(), if duration.num_days() > 1 { "s" } else { "" })
        } else if duration.num_hours() > 0 {
            format!("{}h", duration.num_hours())
        } else if duration.num_minutes() > 0 {
            format!("{}m", duration.num_minutes())
        } else {
            format!("{}s", duration.num_seconds())
        }
    }
}
