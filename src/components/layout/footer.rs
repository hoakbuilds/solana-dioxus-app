use dioxus::prelude::*;
use dioxus_free_icons::prelude::*;
use solana_wallet_adapter_dioxus::use_connection;

/// The application footer.
#[component]
pub fn Footer() -> Element {
    let mut solana_slot = use_signal(|| 0);
    let mut solana_tps = use_signal(|| 0.0);

    use_future(move || async move {
        loop {
            //let now = gloo_timers::.duration_since(UNIX_EPOCH).unwrap();
            //log::info!("Now: {:?}", now);
            //time_now.set(now.as_secs());
            gloo_timers::future::TimeoutFuture::new(500).await;
        }
    });

    use_future(move || async move {
        let connection = use_connection();
        loop {
            let recent_perf = match connection.client.get_recent_performance_samples(None).await {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Failed to fetch recent performance samples. {:?}", e);
                    continue;
                }
            };

            if let Some(perf_sample) = recent_perf.iter().max_by_key(|r| r.slot) {
                log::info!("{:?}", perf_sample);
                solana_tps.set(
                    perf_sample.num_transactions as f64 / perf_sample.sample_period_secs as f64,
                );
            }

            gloo_timers::future::TimeoutFuture::new(5_000).await;
        }
    });

    use_future(move || async move {
        let connection = use_connection();
        loop {
            match connection.client.get_slot().await {
                Ok(r) => {
                    solana_slot.set(r);
                }
                Err(e) => {
                    log::error!("Failed to fetch latest slot");
                    continue;
                }
            };

            log::info!("{:?}", solana_slot.peek());

            gloo_timers::future::TimeoutFuture::new(1_000).await;
        }
    });

    let solana_tps_text = format!("{:.2}", solana_tps());
    let solana_slot_text = format!("{}", solana_slot());

    rsx! {
        footer {
            class: "flex text-center items-center py-3 px-4 md:px-8 text-xs bg-transparent justify-between",
            div {
                class: "md:flex items-center",button {
                    class: "text-white/50 hover:text-white",
                    a {
                        class:"h-6 w-6 px-1 py-1.5 text-white/50 hover:text-white flex items-center",
                        href: "https://blog.anvil.center",
                        Icon {
                            width: 16,
                            height: 16,
                            icon: dioxus_bootstrap_icons::BsFileEarmarkTextFill
                        }
                    }
                }
                button {
                    class: "text-white/50 hover:text-white",
                    a {
                        class:"h-6 w-6 px-1 py-1.5 text-white/50 hover:text-white flex items-center",
                        href: "https://x.com/anvil",
                        Icon {
                            width: 16,
                            height: 16,
                            icon: dioxus_bootstrap_icons::BsX
                        }
                    }
                }
                button {
                    class: "text-white/50 hover:text-white",
                    a {
                        class:"h-6 w-6 px-1 py-1.5 text-white/50 hover:text-white flex items-center",
                        href: "",
                        Icon {
                            width: 16,
                            height: 16,
                            icon: dioxus_bootstrap_icons::BsTelegram
                        }
                    }
                }

            }
            div {
                class: "flex"
            }
            div {
                class: "flex",
                div {
                    class: "text-white/50 hover:text-white flex-1 flex flex-row",
                    span {
                        class: "flex items-center space-x-2 px-1 text-white/50",
                        "Slot"
                    }
                    span {
                        class: "flex items-center space-x-2 px-1 text-white/50",
                        {solana_slot_text}
                    }
                }
                div {
                    class: "text-white/50 hover:text-white flex-1 flex flex-row",
                    span {
                        class: "flex items-center space-x-2 px-1 text-white/50",
                        "TPS"
                    }
                    span {
                        class: "flex items-center space-x-2 px-1 text-white/50",
                        {solana_tps_text}
                    }
                }
            }
        }
    }
}
