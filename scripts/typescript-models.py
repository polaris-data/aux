"""Optional TypeScript derives for generated serde models."""
import re


def strip_annotations(source):
    # rustfmt may expand cfg_attr over several lines. Parse balanced attributes,
    # ignoring square brackets inside TypeScript string literals.
    matches=list(re.finditer(r'(?m)^[ \t]*#\[cfg_attr\(\s*feature\s*=\s*"typescript"',source))
    for match in reversed(matches):
        start=source.index('[',match.start());depth=0;quoted=False;escaped=False
        for end in range(start,len(source)):
            char=source[end]
            if quoted:
                if escaped:escaped=False
                elif char=='\\':escaped=True
                elif char=='"':quoted=False
            elif char=='"':quoted=True
            elif char=='[':depth+=1
            elif char==']':
                depth-=1
                if depth==0:break
        end+=1
        if source[end:end+1]=='\n':end+=1
        source=source[:match.start()]+source[end:]
    return source


def lossless_type(typ):
    if typ.startswith('Option<'):return '('+lossless_type(typ[7:-1])+') | null'
    if typ.startswith('Vec<'):return 'Array<'+lossless_type(typ[4:-1])+'>'
    if 'Map<' in typ:
        inner=typ[typ.index('<')+1:-1];depth=0
        for index,char in enumerate(inner):
            if char=='<':depth+=1
            elif char=='>':depth-=1
            elif char==',' and depth==0:return '{ [key: string]: '+lossless_type(inner[index+1:].strip())+' }'
    if typ=='serde_json::Number':return 'number | bigint | string'
    return 'LosslessValue'


def preceding_attributes(lines):
    blocks=[];index=len(lines)-1
    while index>=0:
        last=index
        if lines[index].strip().endswith(')]'):
            while index>=0 and not lines[index].lstrip().startswith('#['):index-=1
        elif not lines[index].lstrip().startswith('#['):break
        if index<0:break
        blocks.append('\n'.join(lines[index:last+1]));index-=1
    return blocks


def annotate(source):
    """Keep Rust and TS declarations generated from the same serde representation."""
    source=strip_annotations(source)
    source=re.sub(r'(?m)^([ \t]*)(#\[derive\([^\]]*\b(?:Serialize|Deserialize)\b[^\]]*\)\])', r'\1#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]\n\1#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]\n\1\2', source)
    aliases=dict(re.findall(r'pub type (\w+) = (serde_json::Number);',source))
    out=[]
    for line in source.splitlines():
        match=re.match(r'(\s*)pub \w+:\s*(.+),\s*$',line)
        if match:
            attrs=preceding_attributes(out)
            default=any('default' in attr and '#[serde(' in attr for attr in attrs)
            flatten=any('serde(flatten)' in attr for attr in attrs)
            typ=match[2]
            for alias,target in aliases.items():typ=re.sub(r"\b"+alias+r"\b",target,typ)
            if 'serde_json::Number' in typ or 'serde_json::Value' in typ or 'serde_json::Map' in typ:
                if flatten:out.append(match[1]+'#[cfg_attr(feature = "typescript", ts(skip))]')
                else:
                    optional=', optional = nullable' if typ.startswith('Option<') or default else ''
                    out.append(match[1]+f'#[cfg_attr(feature = "typescript", ts(type = "{lossless_type(typ)}"{optional}))]')
            elif default and not flatten:out.append(match[1]+'#[cfg_attr(feature = "typescript", ts(optional = nullable))]')
        out.append(line)
    return '\n'.join(out)+'\n'
