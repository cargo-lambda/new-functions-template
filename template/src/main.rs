{%- if http_function -%}
use lambda_http::{run, service_fn, tracing, Error};
{%- else -%}
use lambda_runtime::{run, service_fn, tracing, Error};
{% endif %}

{%- if http_function %}
mod http_handler;
use http_handler::function_handler;
{%- elsif event_type %}
mod event_handler;
use event_handler::function_handler;
{% else -%}
mod generic_handler;
use generic_handler::function_handler;
{%- endif %}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
