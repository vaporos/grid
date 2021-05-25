// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(any(
    feature = "batch-store",
    feature = "location",
    feature = "pike",
    feature = "product",
    feature = "purchase-order",
    feature = "batch-submitter",
    feature = "track-and-trace"
))]
use std::sync::Arc;

use diesel::r2d2::{ConnectionManager, Pool};

#[cfg(feature = "batch-store")]
use crate::batches::{store::diesel::DieselBatchStore, BatchStore};
#[cfg(feature = "location")]
use crate::locations::{store::diesel::DieselLocationStore, LocationStore};
#[cfg(feature = "pike")]
use crate::pike::{store::diesel::DieselPikeStore, PikeStore};
#[cfg(feature = "product")]
use crate::products::{store::diesel::DieselProductStore, ProductStore};
#[cfg(feature = "purchase-order")]
use crate::purchase_order::{store::diesel::DieselPurchaseOrderStore, PurchaseOrderStore};
#[cfg(feature = "schema")]
use crate::schemas::{store::diesel::DieselSchemaStore, SchemaStore};
#[cfg(feature = "batch-submitter")]
#[cfg(feature = "track-and-trace")]
use crate::track_and_trace::{store::diesel::DieselTrackAndTraceStore, TrackAndTraceStore};

#[derive(Clone)]
pub struct StoreState {
    #[cfg(feature = "batch-store")]
    pub batch_store: Arc<dyn BatchStore>,
    #[cfg(feature = "location")]
    pub location_store: Arc<dyn LocationStore>,
    #[cfg(feature = "pike")]
    pub pike_store: Arc<dyn PikeStore>,
    #[cfg(feature = "product")]
    pub product_store: Arc<dyn ProductStore>,
    #[cfg(feature = "purchase-order")]
    pub purchase_order_store: Arc<dyn PurchaseOrderStore>,
    #[cfg(feature = "schema")]
    pub schema_store: Arc<dyn SchemaStore>,
    #[cfg(feature = "track-and-trace")]
    pub tnt_store: Arc<dyn TrackAndTraceStore>,
}

#[allow(clippy::redundant_clone)]
impl StoreState {
    #[allow(unused_variables)] // needed when connection_pool isn't used because all features disabled
    pub fn with_pg_pool(
        connection_pool: Pool<ConnectionManager<diesel::pg::PgConnection>>,
    ) -> Self {
        #[cfg(feature = "batch-store")]
        let batch_store = Arc::new(DieselBatchStore::new(connection_pool.clone()));
        #[cfg(feature = "location")]
        let location_store = Arc::new(DieselLocationStore::new(connection_pool.clone()));
        #[cfg(feature = "pike")]
        let pike_store = Arc::new(DieselPikeStore::new(connection_pool.clone()));
        #[cfg(feature = "product")]
        let product_store = Arc::new(DieselProductStore::new(connection_pool.clone()));
        #[cfg(feature = "purchase-order")]
        let purchase_order_store = Arc::new(DieselPurchaseOrderStore::new(connection_pool.clone()));
        #[cfg(feature = "schema")]
        let schema_store = Arc::new(DieselSchemaStore::new(connection_pool.clone()));
        #[cfg(feature = "track-and-trace")]
        let tnt_store = Arc::new(DieselTrackAndTraceStore::new(connection_pool));

        Self {
            #[cfg(feature = "batch-store")]
            batch_store,
            #[cfg(feature = "location")]
            location_store,
            #[cfg(feature = "pike")]
            pike_store,
            #[cfg(feature = "product")]
            product_store,
            #[cfg(feature = "purchase-order")]
            purchase_order_store,
            #[cfg(feature = "schema")]
            schema_store,
            #[cfg(feature = "track-and-trace")]
            tnt_store,
        }
    }

    #[allow(unused_variables)] // needed when connection_pool isn't used because all features disabled
    pub fn with_sqlite_pool(
        connection_pool: Pool<ConnectionManager<diesel::sqlite::SqliteConnection>>,
    ) -> Self {
        #[cfg(feature = "batch-store")]
        let batch_store = Arc::new(DieselBatchStore::new(connection_pool.clone()));
        #[cfg(feature = "location")]
        let location_store = Arc::new(DieselLocationStore::new(connection_pool.clone()));
        #[cfg(feature = "pike")]
        let pike_store = Arc::new(DieselPikeStore::new(connection_pool.clone()));
        #[cfg(feature = "product")]
        let product_store = Arc::new(DieselProductStore::new(connection_pool.clone()));
        #[cfg(feature = "purchase-order")]
        let purchase_order_store = Arc::new(DieselPurchaseOrderStore::new(connection_pool.clone()));
        #[cfg(feature = "schema")]
        let schema_store = Arc::new(DieselSchemaStore::new(connection_pool.clone()));
        #[cfg(feature = "track-and-trace")]
        let tnt_store = Arc::new(DieselTrackAndTraceStore::new(connection_pool));

        Self {
            #[cfg(feature = "batch-store")]
            batch_store,
            #[cfg(feature = "location")]
            location_store,
            #[cfg(feature = "pike")]
            pike_store,
            #[cfg(feature = "product")]
            product_store,
            #[cfg(feature = "purchase-order")]
            purchase_order_store,
            #[cfg(feature = "schema")]
            schema_store,
            #[cfg(feature = "track-and-trace")]
            tnt_store,
        }
    }
}
