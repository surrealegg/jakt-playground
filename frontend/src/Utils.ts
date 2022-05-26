export const classNames = (classes: {
  [className: string]: boolean;
}): string => {
  const result: string[] = [];
  for (const key of Object.keys(classes)) if (classes[key]) result.push(key);
  return result.join(" ");
};

interface GistFile {
  content: string;
}

interface GistResponse {
  files: { [name: string]: GistFile };
}

export async function getGist(
  id: string,
  onSuccess: (content: string) => void
) {
  const response = await fetch(`https://api.github.com/gists/${id}`);
  if (response.status !== 200) return;
  const json: GistResponse = await response.json();
  onSuccess(json.files[Object.keys(json.files)[0]].content);
}
