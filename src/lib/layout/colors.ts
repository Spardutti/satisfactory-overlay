/** Belt colors for items players recognise on sight; everything else gets a stable color from its name. */
const KNOWN: Record<string, string> = {
  Desc_OreIron_C: "#c0614a",
  Desc_IronIngot_C: "#a9b2bc",
  Desc_IronPlate_C: "#d4dae1",
  Desc_IronRod_C: "#8b96a3",
  Desc_IronScrew_C: "#b48ce0",
  Desc_OreCopper_C: "#e08a3c",
  Desc_CopperIngot_C: "#e9a15c",
  Desc_Wire_C: "#f0c15a",
  Desc_Cable_C: "#6f86b0",
  Desc_Stone_C: "#cfc6a8",
  Desc_Cement_C: "#b3b3b3",
  Desc_Coal_C: "#8a8a8a",
  Desc_Water_C: "#5bb3e8",
};

export function itemColor(item: string): string {
  if (KNOWN[item]) return KNOWN[item];
  let hash = 0;
  for (const char of item) hash = (hash * 31 + char.charCodeAt(0)) % 360;
  return `hsl(${hash}, 55%, 62%)`;
}
