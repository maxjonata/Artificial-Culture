// Vendored from maxjonata/artificial-culture-site@f48bfff9417dc2b5acb3ef1c6f5bb8659c103222:scripts/publish-public-release.mjs
// Vendor with release.mjs into a product repository. Run only on trusted default-branch pushes, AFTER build/tests.
import { readFile } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import { isDeepStrictEqual } from 'node:util';
import { parseCommit } from './release.mjs';

const knownProjects = Object.fromEntries(['artificial-culture','artificial-culture-site','goth-client','goth-server'].map(p=>[p,{}]));
let range = 'HEAD';
if (process.env.GITHUB_EVENT_PATH) {
  const event = JSON.parse(await readFile(process.env.GITHUB_EVENT_PATH,'utf8'));
  if (process.env.GITHUB_EVENT_NAME !== 'push' || event.deleted) throw Error('publish only runs on a trusted push');
  if (/^[0-9a-f]{40}$/.test(event.before) && !/^0+$/.test(event.before)) range = `${event.before}..HEAD`;
}
const messages = execFileSync('git',['log','--reverse','--format=%B%x00',range],{encoding:'utf8'}).split('\0').map(s=>s.trim()).filter(Boolean);
const notes = messages.map(m=>parseCommit(m,knownProjects)).filter(Boolean);
if (!notes.length) { console.log('No major/minor release in this push.'); process.exit(0); }
const token = process.env.PUBLIC_SITE_TOKEN;
const repository = process.env.PUBLIC_SITE_REPOSITORY;
if (!token || !repository || !/^[\w.-]+\/[\w.-]+$/.test(repository)) throw Error('Configure PUBLIC_SITE_TOKEN and PUBLIC_SITE_REPOSITORY; release notes were not published.');
const headers = { Authorization:`Bearer ${token}`,Accept:'application/vnd.github+json','X-GitHub-Api-Version':'2022-11-28','Content-Type':'application/json' };
for (const note of notes) {
  const path = `releases/${note.project}/v${note.version}.json`;
  const url = `https://api.github.com/repos/${repository}/contents/${path}`;
  const existing = await fetch(url,{headers});
  if (existing.ok) {
    const file = await existing.json();
    const previous = JSON.parse(Buffer.from(file.content,'base64').toString('utf8'));
    if (!isDeepStrictEqual(previous,note)) throw Error(`Release ${note.project}@${note.version} is immutable; contents conflict.`);
    console.log(`Already published ${note.project}@${note.version}.`); continue;
  }
  if (existing.status !== 404) throw Error(`Website access failed: HTTP ${existing.status}.`);
  const publicMessage = `release(${note.project}): v${note.version}\n\n\`\`\`patch-notes\n${JSON.stringify(note,null,2)}\n\`\`\``;
  const result = await fetch(url,{method:'PUT',headers,body:JSON.stringify({branch:'main',message:publicMessage,content:Buffer.from(JSON.stringify(note,null,2)+'\n').toString('base64')})});
  if (!result.ok) throw Error(`Website commit failed: HTTP ${result.status}.`);
  console.log(`Submitted ${note.project}@${note.version}; website CI must still pass before deployment.`);
}

