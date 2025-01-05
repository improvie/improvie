import { untrack } from "svelte";

const MOBILE_BREAKPOINT = 768;

export class IsMobile {
  #current = $state<boolean>(false);

  constructor() {
    $effect(() => {
      return untrack(() => {
        // @ts-ignore: this is work
        const mql = globalThis.window.matchMedia(
          `(max-width: ${MOBILE_BREAKPOINT - 1}px)`,
        );
        const onChange = () => {
          // @ts-ignore: this is work
          this.#current = globalThis.window.innerWidth < MOBILE_BREAKPOINT;
        };
        mql.addEventListener("change", onChange);
        onChange();
        return () => {
          mql.removeEventListener("change", onChange);
        };
      });
    });
  }

  get current() {
    return this.#current;
  }
}
