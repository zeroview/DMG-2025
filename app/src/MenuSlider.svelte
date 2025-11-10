<script lang="ts">
  let {
    value,
    min,
    max,
    step,
    valueLabelCallback,
  }: {
    value: { value: number; effect: (val: number) => void };
    min: number;
    max: number;
    step: number;
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

<div class="slider-row">
  <input
    type="range"
    bind:value={value.value}
    {min}
    {max}
    {step}
    style="width: 250px"
  />
  <p style="width: 5rem;">{valueLabel}</p>
</div>
