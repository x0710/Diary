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
    pub fn read_day(&mut self, date: Date) -> Result<Option<GuiDayState>, Error> {
        async_std::task::block_on(async {
            Ok(self.executor.conn_mut().read_day(date).await?
                .map(GuiDayState::from))
        })
    }

    /// 更新某一天的日记内容，如果当天有就覆盖，没有就新增
    pub fn update_day(&mut self, day: &GuiDayState) -> Result<(), Error> {
        async_std::task::block_on(async {
            self.executor.conn_mut().add_day(&day.into()).await?;
            Ok(())
        })
    }

    /// 删除某一天的日记
    pub fn delete_day(&mut self, date: Date) -> Result<(), Error> {
        async_std::task::block_on(async {
            self.executor.conn_mut().remove_day(date).await?;
            Ok(())
        })
    }

}
