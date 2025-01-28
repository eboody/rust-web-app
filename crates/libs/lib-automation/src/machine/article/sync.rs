//#![allow(unused)]
//use crate::prelude::*;
//use model::SubstackDraft;
//use statum::{machine, state};
//
//#[state]
//#[derive(Debug)]
//pub enum SyncState {
//    Synced,
//    Unsynced,
//}
//
//#[machine]
//pub struct SyncMachine<S: SyncState> {
//    article: model::Articles,
//    mm: ModelManager,
//}
//
//impl<S: SyncState> SyncMachine<S> {
//    pub async fn sync(&self) -> Result<()> {
//        let draft = SubstackDraft::get_by_article_id(self.article.id, &self.mm).await?;
//        let mut draft = draft.unwrap_or_else(|| SubstackDraft::new(self.article.id));
//
//        // If the draft is already synced, return early
//        if draft.sync_status == "synced" {
//            return Ok(());
//        }
//
//        // If the draft is unsynced, sync it
//        if draft.sync_status == "unsynced" {
//            // Sync the draft
//            draft.sync_status = "syncing".to_string();
//            draft.save(&self.mm).await?;
//
//            // Sync the draft
//            let substack_draft = self.sync_draft(&draft).await?;
//
//            // Update the draft
//            draft.sync_status = "synced".to_string();
//            draft.last_synced_at = OffsetDateTime::now();
//            draft.save(&self.mm).await?;
//
//            // Update the article
//            self.article.substack_draft_id = Some(substack_draft.id);
//            self.article.save(&self.mm).await?;
//
//            return Ok(());
//        }
//
//        // If the draft is in an unknown state, return an error
//        Err(Error::new("Unknown draft sync status"))
//    }
//
//    async fn sync_draft(&self, draft: &SubstackDraft) -> Result<SubstackDraft, Error> {
//        // Sync the draft
//        let substack_draft = SubstackDraft::sync(draft, &self.mm).await?;
//
//        // Return the synced draft
//        Ok(substack_draft)
//    }
//}
