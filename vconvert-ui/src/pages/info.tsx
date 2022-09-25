import { Link } from "@solidjs/router";

export default function Info() {
	return (
		<div class="text-white">
			<p class="first-letter:text-2xl first-letter:font-semibold text-neutral-300">
				<span class="text-pink-200 text-lg">VConvert</span> is a tool which
				allows you to convert vulnus maps into soundspace maps, and vice versa!
			</p>
			<p class="text-neutral-300 mt-2">
				To get started, either start converting <span class="font-semibold text-pink-200" >from</span> a Soundspace map, or a Vulnus
				map
			</p>
			<hr class="border-pink-500/60 rounded-2xl border-2 my-4" />
			<div class="flex flex-col space-y-4 py-10 items-center">
				<Link href="/SoundSpace">
					<button class="bg-pink-300/80 shadow-xl p-2 px-4 rounded-xl text-neutral-800 hover:bg-pink-300 transition-colors focus:ring-pink-600 focus:ring-2">
						SoundSpace
					</button>
				</Link>
				<Link href="/Vulnus">
					<button class="bg-fuchsia-300/80 shadow-xl p-2 px-4 rounded-xl text-neutral-800 hover:bg-fuchsia-300 focus:ring-fuchsia-600 focus:ring-2 transition-colors ">
						Vulnus
					</button>
				</Link>
			</div>
		</div>
	);
}
