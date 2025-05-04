export class OpenStore {
  private _open: boolean = $state(false);

  constructor(open?: boolean) {
    if (open !== undefined) {
      this._open = open;
    }
  }

  open() {
    this._open = true;
  }

  close() {
    this._open = false;
  }

  toggle() {
    this._open = !this._open;
  }

  get state(): boolean {
    return this._open;
  }

  set state(value: boolean) {
    this._open = value;
  }
}

export const settingsStore = new OpenStore();
export const sidebarToggler: {
  toggle: () => void;
  isMobile: () => boolean;
} = $state({
  toggle: () => {},
  isMobile: () => false,
});
