use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use tradingview::lightweight_charts::IChartApi;
use wasm_bindgen::JsValue;

#[component]
pub fn Chart() -> Element {
    // <div id="tv_chart_container" class="TradingViewChart"></div>
    let mut initial_render = use_signal(|| false);
    let mut chart = use_signal(|| JsValue::default());

    use_future(move || async move {
        TimeoutFuture::new(1_000).await;
        if initial_render() == false {
            tradingview::initialize();
            log::info!("First render of Chart");
            initial_render.set(true);

            let options = tradingview::lightweight_charts::ChartOptions {
                width: 750,
                height: 600,
                right_price_scale: tradingview::lightweight_charts::PriceScaleOptions {
                    visible: true,
                    border_visible: false,
                    ..Default::default()
                },
                left_price_scale: tradingview::lightweight_charts::PriceScaleOptions {
                    visible: false,
                    border_visible: false,
                    ..Default::default()
                },
                time_scale: tradingview::lightweight_charts::PriceScaleOptions {
                    visible: true,
                    border_visible: false,
                    ..Default::default()
                },
                ..Default::default()
            };

            // let container = tradingview::get_container("tv_chart_container");

            // chart.set(Some(tradingview::lightweight_charts::createChart(
            //     container.into(),
            //     options,
            // )));

            return;
        }
        log::info!("Second render of Chart");
    });

    let tv_widget = &tradingview::tv_widget;

    if let Some(a) = tv_widget.as_string() {
        log::info!("Test: {}", a);
    } else {
        log::info!("No test");
    }

    let base_symbol = "SOL";
    let quote_symbol = "USDC";

    let price_text = format!("{} {}", "85.22", "$");
    let percent_change_text = format!("{} {}", "-0.05", "%");

    rsx! {
        div {
            class: "border border-blue-200 rounded-2xl p-2 shadow dark:bg-black/[.25]",
            div {
                class: "flex-1 flex flex-col px-1 md:px-2 lg:px-3",
                div {
                    class: "flex-1 flex flex-row px-1 md:px-2 lg:px-3",
                    div {
                        class: "flex-1 flex flex-row px-1 md:px-2 lg:px-3",
                        span {
                            class: "font-semibold justify-center items-center fill-current h-[24px] lg:h-[30px] px-1 md:px-2 lg:px-3 text-md lg:text-lg",
                            {base_symbol}
                        }
                        span {
                            class: "font-semibold justify-center items-center fill-current h-[24px] lg:h-[30px] px-1 md:px-2 lg:px-3 text-md lg:text-lg",
                            {quote_symbol}
                        }
                    }
                    div {
                        class: "flex-1 flex flex-col px-1 md:px-2 lg:px-3",
                        span {
                            class: "font-semibold justify-center items-center fill-current h-[24px] lg:h-[30px] px-1 md:px-2lg:px-3 text-sm lg:text-md",
                            {"timeframe selector"}
                        }
                    }
                }
                div {
                    class: "flex-1 flex flex-row px-1 md:px-2 lg:px-3",
                    span {
                        class: "font-semibold justify-center items-center fill-current h-[24px] lg:h-[30px] px-1 md:px-2 lg:px-3 text-sm lg:text-md",
                        {price_text}
                    }
                    span {
                        class: "font-semibold justify-center items-center fill-current h-[24px] lg:h-[30px] px-1 md:px-2 lg:px-3 text-xs lg:text-sm",
                        {percent_change_text}
                    }
                }
                div {
                    class: "border border-gray-200 rounded-2xl p-2 shadow dark:bg-gray-900 dark:border-gray-500",
                    div {
                        id: "tv_chart_container",
                        class: "flex w-full h-full"
                    }
                }
            }
        }
    }
}
