use duktape-rs::DukContext;
use rand;
use rand::seq::SliceRandom;
use regex::Regex;

static ASCII_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/*
def get_random_name(l):
    return random.choice(string.ascii_lowercase) + \
           ''.join(random.sample(string.ascii_letters + string.digits, l - 1))
*/
fn get_random_name(l: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    ASCII_LOWERCASE
        .choose_multiple(&mut rng, l)
        .cloned()
        .collect::<String>()
}

struct NamesDict {
    pub debug_messages: String,
    pub decrypted_codes: String,
    pub resoult: String,
    pub scrambled: String,
    pub workflow: String,
}

impl NamesDict {
    fn new(js_enc: &str) -> Self {
        let workflow_re =
            Regex::new(r"function ub98484234\(.+?[^a-zA-Z0-9_]eval\(([a-zA-Z0-9_]+)\);").unwrap();
        let caps = dbg!(workflow_re.captures(&js_enc)).unwrap();
        let workflow = caps.get(1).unwrap().as_str().to_string();
        NamesDict {
            debug_messages: get_random_name(8),
            decrypted_codes: get_random_name(8),
            resoult: get_random_name(8),
            scrambled: get_random_name(8),
            workflow: workflow,
        }
    }
}

fn run_secret(mut js_enc: String) -> String {
    let nd = NamesDict::new(&js_enc);
    let js_dom = format!(
        "
        {debugMessages} = {{{decryptedCodes}: []}};
        if (!this.window) {{window = {{}};}}
        if (!this.document) {{document = {{}};}}
        ",
        debugMessages = nd.debug_messages,
        decryptedCodes = nd.decrypted_codes,
    );
    let js_patch = format!(
        r#"
        {debugMessages}.{decryptedCodes}.push({workflow});
        var patchCode = function(workflow) {{
            var testVari = /(\w+)=(\w+)\([\w\+]+\);.*?(\w+)="\w+";/.exec(workflow);
            if (testVari && testVari[1] == testVari[2]) {{
                {workflow} += testVari[1] + "[" + testVari[3] + "] = function() {{return true;}};";
            }}
        }};
        patchCode({workflow});
        var subWorkflow = /(?:\w+=)?eval\((\w+)\)/.exec({workflow});
        if (subWorkflow) {{
            var subPatch = (
                "{debugMessages}.{decryptedCodes}.push('sub workflow: ' + subWorkflow);" +
                "patchCode(subWorkflow);"
            ).replace(/subWorkflow/g, subWorkflow[1]) + subWorkflow[0];
            {workflow} = {workflow}.replace(subWorkflow[0], subPatch);
        }}
        eval({workflow});
        "#,
        debugMessages = nd.debug_messages,
        decryptedCodes = nd.decrypted_codes,
        workflow = nd.workflow,
    );

    let js_debug = format!(
        "
        var {scrambled} = ub98484234;
        ub98484234 = function(p1, p2, p3) {{
            try {{
                var resoult = {scrambled}(p1, p2, p3);
                {debugMessages}.{resoult} = resoult;
            }} catch(e) {{
                {debugMessages}.{resoult} = e.message;
            }}
            return {debugMessages};
        }};
        ",
        debugMessages = nd.debug_messages,
        resoult = nd.resoult,
        scrambled = nd.scrambled,
    );
    let to_replace = format!("eval({workflow});", workflow = nd.workflow);
    
    let js_enc = js_enc.replace(&to_replace, &js_patch);

    let js_md5 = include_str!("md5.min.js");

    let mut ctx = DukContext::new();
    let _ = ctx.eval_string(js_md5).unwrap();
    let _ = ctx.eval_string(js_dom).unwrap();
    let _ = ctx.eval_string(js_enc).unwrap();
    let _ = ctx.eval_string(js_debug).unwrap();
    let res = ctx.eval_string("ub98484234({vid}, {did}, {tt})",
                              vid = unimplemented!(),
                              did = unimplemented!(),
                              tt  = unimplemented!(),
                              ).unwrap();
    String::new()
}

#[test]
fn test_secret() {
    run_secret(String::from("function ub98484234(hj{eval(abc));"));
} // function ub98484234\(.+?[^a-zA-Z0-9_]eval\(([a-zA-Z0-9_]+)\);
