import type { Content } from '$bindings/item';
import type { RuleFormat } from '$bindings/rule';
import { action_update_content_by_used, action_update_playlist_by_used } from '$lib/action/recents';
import { action_get_rules_format, action_get_rules_format_with_shuffle } from '$lib/action/rules';
import { getLocalStorageOrDefault, setLocalStorage } from '$lib/local-storage';
import { contents } from './items/content';

export class Tracker {
  public current_track_id: string | undefined = $state();
  public play_rules: RuleFormat[] = $state([]);
  public current_rule_idx: number = $state(0);
  public external_open: boolean = $state(false);
  public paused: boolean = $state(false);
  public currentTime: number = $state(0);

  public is_looping: boolean = $state(false);
  public volume: number = $state(0.5);

  public init() {
    this.is_looping
      = getLocalStorageOrDefault('is_looping', 'false') === 'true';
    this.volume = Number(getLocalStorageOrDefault('volume', '0.5'));

    $effect(() => {
      setLocalStorage('is_looping', this.is_looping.toString());
    });

    $effect(() => {
      setLocalStorage('volume', this.volume.toString());
    });
  }

  public set_single_content(id: string) {
    action_update_content_by_used(id);

    const prev_track_id = this.current_track_id;
    this.clear_track();

    this.current_track_id = id;
    if (prev_track_id === id) {
      this.paused = false;
    }
  }

  public clear_track() {
    this.play_rules = [];
    this.current_rule_idx = 0;
    this.current_track_id = undefined;
    this.currentTime = 0;
    this.paused = true;
  }

  public get_current_content(): Content | undefined {
    if (this.current_track_id === undefined) {
      return undefined;
    }
    return contents.get(this.current_track_id);
  }

  public reset_track() {
    const id = this.current_track_id;
    this.clear_track();
    this.current_track_id = id;
    this.paused = false;
  }

  public toggle_external_open() {
    this.external_open = !this.external_open;
  }

  public toggle_loop() {
    this.is_looping = !this.is_looping;
  }

  public toggle_pause() {
    this.paused = !this.paused;
  }

  // Section: Playlist

  public is_playlist(): boolean {
    return this.play_rules.length > 0;
  }

  private set_rules(rules: RuleFormat[]) {
    const prev_track_id = this.current_track_id;
    this.clear_track();

    this.play_rules = rules;
    if (rules.length > 0) {
      this.update_current_track();
      if (prev_track_id === undefined) {
        this.paused = true;
      }
    }
    else {
      this.current_track_id = undefined;
    }
  }

  public async set_rules_by_type(playlist_id: string): Promise<void> {
    action_update_playlist_by_used(playlist_id);

    const formats = await action_get_rules_format(playlist_id);
    this.set_rules(formats);
  }

  public async set_rules_by_type_shuffle(playlist_id: string): Promise<void> {
    action_update_playlist_by_used(playlist_id);

    const formats = await action_get_rules_format_with_shuffle(playlist_id);
    this.set_rules(formats);
  }

  public update_current_track() {
    if (this.is_playlist()) {
      this.currentTime = 0;
      this.current_track_id = this.play_rules[this.current_rule_idx].content_id;
      this.paused = false;
    }
  }

  public set_current_track(idx: number) {
    if (idx < 0 || idx >= this.play_rules.length) {
      return;
    }
    this.current_rule_idx = idx;
    this.update_current_track();
  }

  public next(): boolean {
    if (!this.is_playlist()) {
      this.reset_track();
      return this.is_looping;
    }
    if (this.current_rule_idx < this.play_rules.length - 1) {
      this.current_rule_idx++;
      this.update_current_track();
      return true;
    }
    else if (this.is_looping) {
      this.current_rule_idx = 0;
      this.update_current_track();
      return true;
    }

    return false;
  }

  public previous(): boolean {
    if (!this.is_playlist()) {
      this.reset_track();
      return this.is_looping;
    }
    if (this.current_rule_idx > 0) {
      this.current_rule_idx--;
      this.update_current_track();
      return true;
    }
    else if (this.is_looping) {
      this.current_rule_idx = this.play_rules.length - 1;
      this.update_current_track();
      return true;
    }
    return false;
  }
}

export const tracker = new Tracker();
