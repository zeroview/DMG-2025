<script lang="ts">
  let {
    value,
    min,
    max,
    step,
    label,
    valueLabelCallback,
  }: {
    value: { value: number; effect: (val: number) => void };
    min: number;
    max: number;
    step: number;
    label: string;
    valueLabelCallback?: (value: number) => string;
  } = $props();

  let valueLabel = $derived.by(() => {
    value.effect(value.value);
    if (valueLabelCallback === undefined) {
      return value.value.toString();
    } else {
      return valueLabelCallback(value.value);
    }
  });
</script>

<div class="menu-row">
  <p style="text-align:right">{label}</p>
  <input
    type="range"
    bind:value={value.value}
    {min}
    {max}
    {step}
    style="width: 250px"
  />
  <p>{valueLabel}</p>
</div>
