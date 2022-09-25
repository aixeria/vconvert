import { Link, useLocation } from "@solidjs/router";
import { createMemo, Show } from "solid-js";

export default function Nav() {
    let r = useLocation();
    const pathname = createMemo(() => r.pathname);

    let render_additional = () => {
        return pathname() !== "/"
    }

    return (
        <div class="w-full" >

            <div class="bg-pink-400/10 text-white shadow-xl border-pink-600/20 border m-4 rounded-xl items-center p-4 flex flex-row space-x-2" >

                <Link href="/" class="hover:text-pink-300 transition-colors" ><h1 class="text-3xl first-letter:font-semibold" >VConvert</h1></Link>
                <Show when={render_additional()} >
                    <span class="text-3xl text-pink-500/30" >»</span>
                    <span class="text-neutral-200 text-2xl" >{r.pathname.slice(1)}</span>
                </Show>
            </div>
        </div>
    );
  }