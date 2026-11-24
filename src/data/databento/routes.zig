const Operation = @import("../../core/routes.zig").Operation;

// zig fmt: off
pub const operations = [_]Operation{
    .{ .venue = .databento, .name = "get_range", .verb = "POST", .path = "/v0/timeseries.get_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "download", .verb = "GET", .path = "/", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "timeseries", .verb = "POST", .path = "/v0/timeseries.get_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "security_master_get_range", .verb = "POST", .path = "/v0/security_master.get_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "security_master_get_last", .verb = "POST", .path = "/v0/security_master.get_last", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "corporate_actions_get_range", .verb = "POST", .path = "/v0/corporate_actions.get_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "adjustment_factors_get_range", .verb = "POST", .path = "/v0/adjustment_factors.get_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_publishers", .verb = "GET", .path = "/v0/metadata.list_publishers", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_datasets", .verb = "GET", .path = "/v0/metadata.list_datasets", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_schemas", .verb = "GET", .path = "/v0/metadata.list_schemas", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_fields", .verb = "GET", .path = "/v0/metadata.list_fields", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_unit_prices", .verb = "GET", .path = "/v0/metadata.list_unit_prices", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_dataset_condition", .verb = "GET", .path = "/v0/metadata.get_dataset_condition", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_dataset_range", .verb = "GET", .path = "/v0/metadata.get_dataset_range", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_record_count", .verb = "POST", .path = "/v0/metadata.get_record_count", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_billable_size", .verb = "POST", .path = "/v0/metadata.get_billable_size", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_cost", .verb = "POST", .path = "/v0/metadata.get_cost", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "resolve", .verb = "POST", .path = "/v0/symbology.resolve", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "submit_job", .verb = "POST", .path = "/v0/batch.submit_job", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_jobs", .verb = "GET", .path = "/v0/batch.list_jobs", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "get_job_details", .verb = "GET", .path = "/v0/batch.get_job_details", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_files", .verb = "GET", .path = "/v0/batch.list_files", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_events", .verb = "GET", .path = "/v0/corporate_actions.list_events", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
    .{ .venue = .databento, .name = "list_enums", .verb = "GET", .path = "/v0/corporate_actions.list_enums", .private = true, .origin = "https://hist.databento.com", .params = &.{} },
};
