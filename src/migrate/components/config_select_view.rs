use crate::migrate::migrate_page::ConfigState;
use leptos::prelude::*;

const CONFIG_COUNT: usize = 6;

#[component]
pub fn ConfigSelectForm(config_state_rw: RwSignal<ConfigState>) -> impl IntoView {
    let config_state = config_state_rw.get();
   
    view! {
        <table class="table table-fixed w-full">
            <colgroup>
                <col style="width: 40%;" />
                <col style="width: 60%;" />
            </colgroup>
            <tbody>
                {(0..CONFIG_COUNT).map(|i| {
                    let (name, label) = config_state.items[i];
                    let value_signal = config_state.values[i];
                   
                    view! {
                        <tr>
                            <td class="text-right">
                                <input
                                    class="checkbox checkbox-info"
                                    type="checkbox"
                                    name=name
                                    prop:checked=value_signal
                                    on:input=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        value_signal.set(checked);
                                    }
                                />
                            </td>
                            <td class="text-left">
                                <label class="ml-2">{label}</label>
                            </td>
                        </tr>
                    }
                })
                .collect_view()
                }
            </tbody>
        </table>
    }
}