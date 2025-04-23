import type { Content } from '$bindings/item';
import type { RuleFormat } from '$bindings/Rule';

export class Tracker {
  current_track_id: string | undefined = $state();
  play_rules: RuleFormat[] = $state([]);
  current_rule_idx: number = $state(0);

  set_single_content(id: string) {
    this.current_track_id = id;
    this.play_rules = [];
    this.current_rule_idx = 0;
  }

  set_rules(rules: RuleFormat[]) {
    this.play_rules = rules;
    this.current_rule_idx = 0;
    if (rules.length > 0) {
      this.current_track_id = rules[0].content_id;
    }
    else {
      this.current_track_id = undefined;
    }
  }

  clear_track() {
    this.current_track_id = undefined;
    this.play_rules = [];
    this.current_rule_idx = 0;
  }

  get_current_content(): Content | undefined {
    if (this.current_track_id === undefined) {
      return undefined;
    }
    // return $contents[this.current_track_id];
  }
}

export const tracker = new Tracker();
