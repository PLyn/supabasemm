use leptos::prelude::*;

#[component]
pub fn ConfigSelectForm(config_items_rw: [RwSignal<(String, bool)>; 7]) -> impl IntoView {
    view! {
        <table class="table table-fixed w-full">
            <colgroup>
                <col style="width: 40%;" />
                <col style="width: 60%;" />
            </colgroup>
            <tbody>
                {config_items_rw.into_iter().map(|item_signal| {
                    let (name, enabled) = item_signal.get();

                    view! {
                        <tr>
                            <td class="text-right">
                                <input
                                    class="checkbox checkbox-info"
                                    type="checkbox"
                                    name={name.clone()}
                                    prop:checked=move || enabled
                                    on:change=move |ev| {
                                        let new_value = event_target_checked(&ev);
                                        item_signal.set((item_signal.get().0, new_value));
                                    }
                                />
                            </td>
                            <td class="text-left">
                                <label class="ml-2">{name.clone()}</label>
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