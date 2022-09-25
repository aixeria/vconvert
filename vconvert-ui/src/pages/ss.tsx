import { createSignal, For, Match, Setter, Show, Switch } from "solid-js";
import { open as openModal } from "@tauri-apps/api/dialog";
import { sep } from "@tauri-apps/api/path";
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "@solidjs/router";

type metaupd = [Setter<{[x:string]:any}>,string];


interface metadata {
    _title: string,
    _artist: string,
    _difficulties: string[],
    _mappers: string[],
    _version: number,
    _music: string,
}

export default function ss() {
	let [hof, sethof] = createSignal<boolean>(false);
	let [export_location, set_export_location] = createSignal<string>("");


    let [artist, set_artist] = createSignal<string>("");
    let [http_url, set_http_url] = createSignal<string>("");
    let [title, set_title] = createSignal<string>("");
    let [ptodif, set_ptodif] = createSignal<{[x:string]:string}>({});
    let [mappers, set_mappers] = createSignal<string[]>([]);

    let navigate = useNavigate();


	let open_dialog = async () => {
		let ret = (await openModal({
			title: "Select Soundspace map(s)",
            multiple:true
		})) as string[] | null;
		if (!ret) {
			return;
		}
		let top : {[x:string]:string} = {};
        for (let p of ret) {
            top[p] = p.slice(p.lastIndexOf(sep)+1,p.lastIndexOf("."));
        }
		set_ptodif(top);
	};
	let open_dialog_export = async () => {
		let ret = (await openModal({
            directory:true,
			defaultPath: export_location().length > 0 ? export_location() : "~",
			title: "Export to folder",
		})) as string | null;
		if (!ret) {
			return;
		}

		set_export_location(ret);
	};

	let do_export = async () => {
		await open_dialog_export();

		if (export_location() === "") {
			return;
		}

        let hu = http_url();
        let pog = hu.lastIndexOf("/")+1;
        let nam = hu.slice(pog);
        let qe = nam.lastIndexOf(".",pog);

        let diff = hof() === true ? [nam.slice(0, qe === -1 ? undefined : qe)] : Object.values(ptodif())

        let metadata: metadata = {
            _title: title(),
            _artist: artist(),
            _difficulties: diff,
            _mappers: mappers(),
            _version: 1,
            _music: "song.mp3",
        }

        if (hof() === true) {
            invoke("ss_to_vulnus_remote", {
                path: hu,
                exportTo: export_location(),
                meta: metadata,
            }).then(()=>{
                navigate("/");
            },e=>{
                console.log(e);
            })
        }
        else {

            invoke("ss_to_vulnus_local", {
                paths: ptodif(),
                exportTo: export_location(),
                meta: metadata,
            }).then(()=>{
                navigate("/");
            },e=>{
                console.log(e);
            })
        }
	};

    let handle_new_mapper = (v: {
        currentTarget: HTMLInputElement;
        target: Element;
    },i:number) => {
        let va = v.currentTarget.value;
        if (va === "") {
            set_mappers(old=> [...old.slice(0,i),...old.slice(i+1)]);
        } else {
            set_mappers(old=> {old[i] = (v.target as any).value; return old} )
        }
    }

	return (
		<div class="text-white">
            <h1 class="text-2xl">File or URL</h1>
            <form class="px-2 text-neutral-300">
                <input
                    checked={hof() === false}
                    onInput={(e) => {
                        sethof(e.target.id === "rh");
                    }}
                    class="mr-2 text-pink-400 focus:ring-pink-300"
                    type="radio"
                    name="hof"
                    id="rf"
                />
                <label for="rf">File</label>
                <br />
                <input
                    checked={hof() === true}
                    onInput={(e) => {
                        sethof(e.target.id === "rh");
                    }}
                    class="mr-2 text-pink-400 focus:ring-pink-300"
                    type="radio"
                    name="hof"
                    id="rh"
                />
                <label for="rh">HTTP</label>
            </form>

            <Switch>
                <Match when={hof() === true}>
                    {/* http */}
                    <h1 class="text-2xl">Remote Map</h1>
                    <div class="mx-2" >

                        <label class="block text-neutral-500" for="http">
                                URL
                        </label>
                        <input
                            oninput={v=>set_http_url(v.currentTarget.value)}
                            placeholder={"HTTP url of map"}
                            class="appearance-none flex-1 w-full text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                            id="http"
                        />
                    </div>
                    </Match>
                <Match when={hof() === false}>
                    {/* file */}

                    <h1 class="text-2xl">Locate Map</h1>

                    <button
                        onClick={open_dialog}
                        class="bg-fuchsia-300/80 shadow-xl mx-2 mb-1 mt-4 p-2 px-4 rounded-xl text-neutral-800 hover:bg-fuchsia-300 transition-colors focus:ring-fuchsia-500 focus:ring-2"
                    >
                        Open file selector
                    </button>
                    <Show when={Object.keys(ptodif()).length > 0 } >
                        <h1 class="text-2xl">Difficulty Mapper</h1>

                        <div class="mx-2 bg-neutral-800 my-2 rounded-lg space-y-2 p-2" >
                            <For each={ Object.entries(ptodif()) }  >{(pd)=>
                            <>
                            <div class="flex space-x-2" >
                                <input
                                    readOnly={true}
                                    value={pd[0].slice(pd[0].lastIndexOf(sep)+1)}
                                    class="appearance-none flex-1 text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                />
                                <input
                                    value={pd[1]}
                                    onFocusOut={(e) => {
                                        set_ptodif(old=>{
                                            return {...old,[pd[0]]:(e.target as any).value}
                                        })
                                    } }
                                    class="appearance-none flex-[.4] text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                />
                            </div>
                            </>}</For>
                        </div>
                    </Show>
                </Match>
            </Switch>

            <h1 class="text-2xl">Meta data</h1>
            <div class="mx-2">
                <div class="flex flex-col" >
                    <div class="flex flex-wrap space-x-2" >

                        <div class="flex-1 min-w-[170px]" >
                            <label class="block text-neutral-500" for="artist">
                            Artist
                            </label>
                            <input
                                oninput={v=>set_artist((v.target as any).value)}
                                placeholder={"the artist of the song"}
                                class="appearance-none flex-1 w-full text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                id="artist"
                                />
                        </div>

                        <div class="flex-1 min-w-[170px]" >

                            <label class="block text-neutral-500" for="title">
                                Song title
                            </label>
                            <input
                                oninput={v=>set_title((v.target as any).value)}
                                placeholder={"the title of the song"}
                                class="appearance-none flex-1 w-full text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                id="title"
                                />
                        </div>
                    </div>
                    <div class="flex flex-col my-2 space-y-2" >

                        <label class="block text-neutral-500">
                            Mappers
                        </label>
                        <For each={mappers()}>{(m,i)=><>
                        
                            <input
                                value={m}
                                onFocusOut={(e)=>handle_new_mapper(e,i())}
                                onsubmit={(e)=>handle_new_mapper(e,i())}
                                onkeydown={(e)=>{
                                    if (e.key === "Enter") {
                                        e.currentTarget.blur()
                                    }
                                }}
                                class="appearance-none flex-1 w-full text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                id="Mappers"
                            />
                        
                        </>}</For>
                        <div class="flex-1 min-w-[170px]" >
                            <input
                                onFocusOut={v=>{let g = (v.target as any).value; (v.target as any).value = ""; if (g === "") {return;}; set_mappers(old=> [...old,g])}}
                                onkeydown={(e)=>{
                                    if (e.key === "Enter") {
                                        e.currentTarget.blur()
                                    }                                
                                }}
                                placeholder={"Type to add a new mapper"}
                                class="appearance-none flex-1 w-full text-neutral-500 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none border-neutral-600 focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-1 shadow-sm border"
                                id="Mappers"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <div class="w-full flex items-end">
                <button
                    disabled={!((title().length > 0 
                        && artist().length > 0 
                        && mappers().length > 0))}
                    onClick={do_export}
                    class="bg-pink-300/80 ml-auto mb-0 mx-2 mt-3 p-2 px-2 rounded-xl disabled:bg-pink-300/40 text-neutral-800 hover:bg-pink-300 transition-colors focus:ring-pink-500 focus:ring-2"
                >
                    Export
                </button>
            </div>
		</div>
	);
}
