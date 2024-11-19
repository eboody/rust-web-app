#[macro_export]
macro_rules! impl_service_trait {
    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        GetService
    ) => {
            #[async_trait::async_trait]
            impl<'a> GetService<'a, $resource> for $service_name<'a> {
                type Id = $id_type;

#[doc = concat!("Fetches a single instance of the resource by ID.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `id`: The unique identifier of the resource, of type [`i64`].")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult) ,"`]<[`LabOrderSet`]> containing the requested resource if found, or a [`ServiceError`] if not.")]
#[doc = ""]
#[doc = "### Example:"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let id: ", stringify!($id_type), " = 1;")]
#[doc = concat!("let resource: ", stringify!($resource) ," = service.get(id).await?;")]
#[doc = concat!("println!(\"", stringify!($resource) ,": {:?}\", resource);")]
#[doc = "```"]
                async fn get(&self, id: Self::Id) -> ServiceResult<$resource> {
                    self.base.get(id).await
                }
            }
        };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PostService
    ) => {


            #[async_trait::async_trait]
            impl<'a> PostService<'a, $resource, $resource_for_create> for $service_name<'a> {
#[doc = concat!("Creates a new instance of the resource.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `resource_for_create`: A reference to the creation struct of type [`", stringify!($resource_for_create), "`].")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult), "`]<[`", stringify!($resource), "`]> containing the created resource if successful, or a [`ServiceError`] if creation fails.")]
#[doc = ""]
#[doc = "### Example:"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let new_resource = ", stringify!($resource_for_create), " { /* fields */ };")]
#[doc = concat!("let created_resource: ", stringify!($resource), " = service.post(&new_resource).await?;")]
#[doc = concat!("println!(\"Created Resource: {:?}\", created_resource);")]
#[doc = "```"]
                async fn post(&self, resource: &$resource_for_create) -> Result<$resource> {
                    self.base.post(resource).await
                }
            }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        DeleteService
    ) => {
            #[async_trait::async_trait]
            impl<'a> DeleteService<'a> for $service_name<'a> {
                type Id = $id_type;

#[doc = concat!("Deletes an existing resource by ID.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `id`: The unique identifier of the resource to delete, of type [`", stringify!($id_type), "`].")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult), "`]<()>: Indicates success with an empty result, or a [`ServiceError`] if deletion fails.")]
#[doc = ""]
#[doc = "### Example:"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let id: ", stringify!($id_type), " = 1;")]
#[doc = "service.delete(id).await?;"]
#[doc = "println!(\"Resource deleted successfully.\");"]
#[doc = "```"]
                async fn delete(&self, id: Self::Id) -> Result<()> {
                    self.base.delete(id).await
                }
            }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PatchService
    ) => {

                #[async_trait::async_trait]
                impl<'a> PatchService<'a, $resource, $resource_for_update> for $service_name<'a> {
                    type Id = $id_type;


#[doc = concat!("Applies a partial update to an existing resource by ID.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `id`: The unique identifier of the resource to update, of type [`", stringify!($id_type), "`].")]
#[doc = concat!("- `resource`: A reference to the update struct, of type `", stringify!($resource_for_update), "`.")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult), "`]<[`", stringify!($resource), "`]> containing the updated resource if successful, or a [`ServiceError`] if the operation fails.")]
#[doc = ""]
#[doc = "### Example:"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let id: ", stringify!($id_type), " = 1;")]
#[doc = concat!("let update_data = ", stringify!($resource_for_update), " { /* fields */ };")]
#[doc = concat!("let updated_resource: ", stringify!($resource), " = service.patch(id, &update_data).await?;")]
#[doc = concat!("println!(\"Updated Resource: {:?}\", updated_resource);")]
#[doc = "```"]

                    async fn patch(
                        &self,
                        id: Self::Id,
                        resource: &$resource_for_update,
                    ) -> Result<$resource> {
                        self.base.patch(id, resource).await
                    }
                }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        PutService
    ) => {
            #[async_trait::async_trait]
            impl<'a> PutService<'a, $resource, $resource_for_create> for $service_name<'a> {
                type Id = $id_type;

#[doc = concat!("Replaces an existing resource or creates it if it does not exist.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `resource`: A reference to the struct used for replacement or creation, of type [`", stringify!($resource_for_create), "`].")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult), "`]<[`", stringify!($resource), "`]> containing the upserted resource if successful, or a [`ServiceError`] if the operation fails.")]
#[doc = ""]
#[doc = "### Example:"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let resource_data = ", stringify!($resource_for_create), " { /* fields */ };")]
#[doc = concat!("let upserted_resource: ", stringify!($resource), " = service.put(&resource_data).await?;")]
#[doc = concat!("println!(\"Upserted Resource: {:?}\", upserted_resource);")]
#[doc = "```"]
                async fn put(&self, resource: &$resource_for_create) -> Result<$resource> {
                    self.base.put(resource).await
                }
            }
    };

    (
        $service_name:ident,
        $resource:ty,
        $resource_for_create:ty,
        $resource_for_update:ty,
        $resource_query_params:ty,
        $id_type:ty,
        FindService
    ) => {
            #[async_trait::async_trait]
            impl<'a> FindService<'a, $resource, $resource_query_params> for $service_name<'a> {
#[doc = concat!("Finds pages of resources based on the provided query parameters.")]
#[doc = ""]
#[doc = "### Parameters:"]
#[doc = concat!("- `params`: The query parameters used for filtering, of type [`", stringify!($resource_query_params), "`].")]
#[doc = ""]
#[doc = "### Returns:"]
#[doc = concat!("- [`", stringify!(ServiceResult), "`]<[`PaginatedResponse`] containing the paginated resources matching the criteria, or a [`ServiceError`] if the operation fails.")]
#[doc = ""]
#[doc = "### Example: Fetching a Single Page"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let params = ", stringify!($resource_query_params), " { /* fields */ };")]
#[doc = concat!("let page: PaginatedResponse<", stringify!($resource), "> = service.find(params).await?;")]
#[doc = "println!(\"Items on this page: {:?}\", page.results);"]
#[doc = "println!(\"Total items available: {}\", page.count);"]
#[doc = "```"]
#[doc = ""]
#[doc = "### Example: Iterating Through All Pages"]
#[doc = "```rust"]
#[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
#[doc = concat!("let params = ", stringify!($resource_query_params), " { /* fields */ };")]
#[doc = "let mut current_page = service.find(params).await?;"]
#[doc = "let mut all_results = current_page.results;"]
#[doc = "while current_page.has_next() {"]
#[doc = "    if let Some(next_page) = current_page.fetch_next_page(&client).await? {"]
#[doc = "        all_results.extend(next_page.results);"]
#[doc = "        current_page = next_page;"]
#[doc = "    } else {"]
#[doc = "        break;"]
#[doc = "    }"]
#[doc = "}"]
#[doc = "println!(\"Total items fetched: {}\", all_results.len());"]
#[doc = "```"]
                async fn find(&self, params: $resource_query_params) -> Result<PaginatedResponse<$resource>> {
                    self.base.find(params).await
                }
            }
    };
}

#[macro_export]
macro_rules! impl_service {
    (
        ServiceName: $service_name:ident,
        Resource: $resource:ty,
        ForCreate: $resource_for_create:ty,
        ForUpdate: $resource_for_update:ty,
        QueryParams: $resource_query_params:ty,
        IdType: $id_type:ty,
        Traits: [$($trait_name:ident),*]
    ) => {
        // Struct-level documentation
        #[doc = concat!("Provides methods for managing [`", stringify!($resource), "`](models::", stringify!($resource), ") resources.")]
        #[doc = ""]
        #[doc = "### Overview"]
        #[doc = concat!("This service allows for operations on [`", stringify!($resource), "`] entities with methods depending on the specified traits.")]
        #[doc = "Refer to the 'Trait Implementations' section to see which methods this service includes:"]
        #[doc = ""]
        $(
            #[doc = concat!("- **[`", stringify!($trait_name), "`](#trait-implementations)**: Provides ", stringify!($trait_name), " functionality for [`", stringify!($resource), "`].")]
        )*
        #[doc = ""]
        #[doc = "### Examples"]
        #[doc = ""]
        #[doc = concat!("#### Example: Initializing `", stringify!($service_name), "`")]
        #[doc = "```rust"]
        #[doc = "let client = Client::new();"]
        #[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
        #[doc = "```"]
        pub struct $service_name<'a> {
            base: BaseService<'a, $resource, $resource_for_create, $resource_for_update>,
        }

        impl<'a> $service_name<'a> {
            // `new` function documentation
            #[doc = concat!("Creates a new instance of [`", stringify!($service_name), "`].")]
            #[doc = ""]
            #[doc = "### Parameters:"]
            #[doc = "- `client`: A reference to the client instance used for making API requests."]
            #[doc = ""]
            #[doc = "### Returns:"]
            #[doc = concat!("- A new instance of `", stringify!($service_name), "` configured to manage [`", stringify!($resource), "`] resources.")]
            #[doc = ""]
            #[doc = "### Example"]
            #[doc = "```rust"]
            #[doc = "let client = Client::new();"]
            #[doc = concat!("let service = ", stringify!($service_name), "::new(&client);")]
            #[doc = "```"]
            pub fn new(client: &'a Client) -> Self {
                Self {
                    base: BaseService::new(client),
                }
            }
        }

        // Implement each specified trait for this service
        $(
            $crate::impl_service_trait!(
                $service_name,
                $resource,
                $resource_for_create,
                $resource_for_update,
                $resource_query_params,
                $id_type,
                $trait_name
            );
        )*
    };
}
