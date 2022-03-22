const string1 = "test";
const string2 = 'test';
const string3 = 'te"st';
const string4 = "te'st";
const string5 = "test\ntest\ntest";
const string6 = `Yet another string primitive`;
const string7 = "This is a very long string which needs \
to wrap across multiple lines because \
otherwise my code is unreadable.";
const string8 = "中文 español English हिन्दी العربية português বাংলা русский 日本語 ਪੰਜਾਬੀ 한국어 தமிழ்";
const string9 = ``;
const string10 = `xx\`x`;
const string11 = `${ foo + 2 }`;
const string12 = ` foo ${ bar + `baz ${ qux }` }`;
const string13 = String.raw`foo`;
const string14 = foo `bar`;
const string15 = `foo
bar
ↂωↂ`;
const string16 = `\``;
const string17 = `${4 + 4} equals 4 + 4`;
const string18 = `This is ${undefined}`;
const string19 = `This is ${NaN}`;
const string20 = `This is ${null}`;
const string21 = `This is ${Infinity}`;
const string22 = "This is ${1/0}";
const string23 = 'This is ${1/0}';
const string24 = "This is ${NaN}";
const string25 = "This is ${null}";
const string26 = `This is ${1/0}`;
const string27 = `This is ${0/0}`;
const string28 = "This is ${0/0}";
const string29 = 'This is ${0/0}';
const string30 = `${4**11}`;
const string31 = `${4**12}`;
const string32 = `${4**14}`;
const string33 = '';
const string34 = '\b';
const string35 = '\f';
const string36 = '\t';
const string37 = '\v';
const string38 = '\n';
const string39 = '\\n';
const string40 = '\\';
const string41 = '\\"';
const string42 = '\'\"';
const string43 = '\\\\';
const string44 = '\x00';
const string45 = '\x00!';
const string46 = '\x001';
const string47 = '\\0';
const string48 = '\\0!';
const string49 = '\x07';
const string50 = '\x07!';
const string51 = '\x071';
const string52 = '\7';
const string53 = '\\7';
const string54 = '\\7!';
const string55 = '\\01';
const string56 = '\x10';
const string57 = '\\x10';
const string58 = '\x1B';
const string59 = '\\x1B';
const string60 = '\uABCD';
const string61 = '\uABCD';
const string62 = '\U000123AB';
const string63 = '\u{123AB}';
const string64 = '\uD808\uDFAB';
const string65 = '\uD808';
const string66 = '\uD808X';
const string67 = '\uDFAB';
const string68 = '\uDFABX';
const string69 = '\x80';
const string70 = '\xFF';
const string71 = '\xF0\x9F\x8D\x95';
const string72 = '\uD801\uDC02\uDC03\uD804';
const string73 = 'π';
const 貓 = '🐈';
const 貓abc = '🐈';
const abc貓 = '🐈';
const string74 = '\u2028';
const string75 = '\u2029';
const string76 = '\uFEFF';
const string77 = '\x10';
const string78 = '\x20';
const string79 = ' ';
const string80 = '\x32';
const string81 = '\x16';
const string82 = '\x06';
const string83 = '\0a';
const string84 = "\"test\"test\"test"
const string85 = "\"test\'test\'test"
const string86 = '\"test\"test\"test';
const string87 = '\'test\'test\'test';
const string88 = '😄';
const string89 = new RegExp("\r").test("\r");
const string90 = new RegExp(" ").test(" ");
const string91 = new RegExp("\x1b").test("[" + "\x1b" + "]");
const string92 = new RegExp("\\x1b").test("\x1b");
const string93 = new RegExp("").test("");
const string94 = '퟿';
const string95 = 'ퟻ';
const string96 = sql`'#ERROR'`;
const string97 = '\u{a0}';
const string98 = "\ud83d\ude00";
const string99 = "\ud83d@\ude00";
// const string97 = '\u{D800}';
// const string97 = '\u{DBFF}';
// const string98 = '\u{DC00}';
// const string99 = '\u{DFFF}';
