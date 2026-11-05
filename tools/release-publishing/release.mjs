// Vendored from maxjonata/artificial-culture-site@f48bfff9417dc2b5acb3ef1c6f5bb8659c103222:scripts/release.mjs
import assert from 'node:assert/strict';

export function semver(value) {
  assert.equal(typeof value, 'string', 'version must be a string');
  assert.match(value, /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/, 'invalid stable semver');
  const parts = value.split('.').map(Number);
  assert(parts.every(Number.isSafeInteger), 'version exceeds safe range');
  return parts;
}
export function compare(a, b) {
  const aa = semver(a), bb = semver(b);
  for (let i = 0; i < 3; i++) if (aa[i] !== bb[i]) return Math.sign(aa[i] - bb[i]);
  return 0;
}
function boundedText(value, name, max = 1200) {
  assert.equal(typeof value, 'string', `${name} must be text`);
  assert(value.trim().length > 0 && value.length <= max, `${name} has invalid length`);
  assert(!/[\u0000-\u0008\u000b\u000c\u000e-\u001f]/.test(value), `${name} contains controls`);
}
function exactKeys(value, keys, name) {
  assert(value && typeof value === 'object' && !Array.isArray(value), `${name} must be an object`);
  assert.deepEqual(Object.keys(value).sort(), [...keys].sort(), `${name} has missing or unexpected fields`);
}
export function validateRelease(note, projects) {
  exactKeys(note, ['schema_version','project','version','kind','date','public','state','pt','en'], 'release');
  assert.equal(note.schema_version, 1, 'unsupported schema');
  assert(Object.hasOwn(projects, note.project), 'unknown project');
  semver(note.version);
  assert(['major','minor'].includes(note.kind), 'only major/minor releases enter this feed');
  assert.equal(note.public, true, 'explicit public approval missing');
  assert(['implemented','experimental','planned','mixed'].includes(note.state), 'unknown state');
  assert.match(note.date, /^\d{4}-\d{2}-\d{2}$/);
  assert.equal(new Date(note.date).toISOString().slice(0, 10), note.date, 'invalid calendar date');
  for (const language of ['pt','en']) {
    const text = note[language];
    exactKeys(text, ['title','changes','limitations'], language);
    boundedText(text.title, `${language}.title`, 160);
    for (const key of ['changes','limitations']) {
      assert(Array.isArray(text[key]) && text[key].length >= 1 && text[key].length <= 20, `invalid ${language}.${key}`);
      text[key].forEach(item => boundedText(item, `${language}.${key}`));
    }
  }
  assert(JSON.stringify(note).length <= 20000, 'release too large');
  return note;
}
export function parseCommit(message, projects) {
  const subject = message.split('\n')[0].trim();
  if (!subject.startsWith('release(')) return null;
  const matched = /^release\(([a-z0-9-]+)\): v(\d+\.\d+\.\d+)$/.exec(subject);
  assert(matched, 'release subject must be release(project): vX.Y.Z');
  const blocks = [...message.matchAll(/^```patch-notes\r?\n([\s\S]*?)^```\s*$/gm)];
  assert.equal(blocks.length, 1, 'exactly one patch-notes JSON block required');
  const note = validateRelease(JSON.parse(blocks[0][1]), projects);
  assert.equal(note.project, matched[1], 'scope differs from public project');
  assert.equal(note.version, matched[2], 'subject differs from public version');
  return note;
}
export function collectReleases(notes, projects) {
  const unique = new Map();
  for (const note of notes) {
    validateRelease(note, projects);
    const key = `${note.project}@${note.version}`;
    if (unique.has(key)) assert.deepEqual(unique.get(key), note, 'published version has conflicting contents');
    unique.set(key, note);
  }
  for (const [project, policy] of Object.entries(projects)) {
    let previous = policy.baseline;
    for (const note of [...unique.values()].filter(n => n.project === project).sort((a,b) => compare(a.version,b.version))) {
      assert(compare(note.version, previous) > 0, 'release must advance its project version');
      const old = semver(previous), next = semver(note.version);
      assert.equal(note.kind, next[0] > old[0] ? 'major' : 'minor', 'kind disagrees with semver change');
      assert(next[0] > old[0] || next[1] > old[1], 'patch-only change is not major/minor');
      previous = note.version;
    }
  }
  return [...unique.values()].sort((a,b) => b.date.localeCompare(a.date) || a.project.localeCompare(b.project) || compare(b.version,a.version));
}
export function validateReleaseChanges(changes, notes) {
  for (const line of changes.split('\n').filter(Boolean)) {
    assert(line.startsWith('A\t'), 'published release files are immutable');
    const file = line.slice(2);
    const match = /^releases\/([a-z0-9-]+)\/v(\d+\.\d+\.\d+)\.json$/.exec(file);
    assert(match, 'unexpected release file path');
    assert(notes.some(n => n.project === match[1] && n.version === match[2]), 'new release file requires a matching release commit with public patch notes');
  }
}
const states = { implemented:'Implementado / Implemented', experimental:'Experimental', planned:'Planejado / Planned', mixed:'Entrega parcial / Partial delivery' };
export function toEntry(note, projects) {
  const entry = { release_key: `${note.project}@${note.version}`, project:note.project, version:note.version, kind:note.kind,
    date:note.date, state:states[note.state], title:`${projects[note.project].label} v${note.version} · ${note.pt.title} / ${note.en.title}` };
  for (const language of ['pt','en']) {
    const text = note[language];
    entry[language] = text.changes.join(' ') + (language === 'pt' ? ' Limites: ' : ' Limitations: ') + text.limitations.join(' ');
  }
  return entry;
}

