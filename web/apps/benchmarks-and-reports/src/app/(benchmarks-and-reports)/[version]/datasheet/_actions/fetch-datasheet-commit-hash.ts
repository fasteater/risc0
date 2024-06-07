import "server-only";

import { tryFetch } from "shared/utils/try-fetch";
import type { Version } from "~/types/version";

export async function fetchDatasheetCommitHash({ version }: { version: Version }) {
  const [error, response] = await tryFetch(
    `https://raw.githubusercontent.com/risc0/ghpages/${version}/dev/datasheet/COMMIT_HASH.txt`,
  );

  // error handling
  if (error || !response.ok) {
    throw error || new Error("Failed to fetch");
  }

  return {
    data: await response.text(),
    fetchedAt: Date.now(),
  };
}
