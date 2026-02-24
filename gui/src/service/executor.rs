use diary_core::base::date::Date;
use diary_core::base::executor::Executor;
use diary_core::base::error::Error;
use crate::model::day::GuiDayState;

pub struct GuiService {
    pub(crate) executor: Executor,
}

impl GuiService {
    pub fn new(executor: Executor) -> Self {
        Self { executor }
    }

    /// 查询某一天的内容
    pub fn read_day(&self, date: Date) -> Result<Option<GuiDayState>, Error> {
        match self.executor.conn().read_day(date) {
            Ok(Some(day)) => Ok(Some(day.into())),
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()), // 转换成统一 Error 类型
        }
    }

    /// 更新某一天的日记内容，如果当天有就覆盖，没有就新增
    pub fn update_day(&self, day: &GuiDayState) -> Result<(), Error> {
        self.executor.conn().add_day(&day.into())?;
        Ok(())
    }

    /// 删除某一天的日记
    pub fn delete_day(&self, date: Date) -> Result<(), Error> {
        self.executor.conn().remove_day(date)?;
        Ok(())
    }

}
