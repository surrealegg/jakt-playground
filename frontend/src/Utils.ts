export const classNames = (classes: {
  [className: string]: boolean;
}): string => {
  const result: string[] = [];
  for (const key of Object.keys(classes)) if (classes[key]) result.push(key);
  return result.join(" ");
};
