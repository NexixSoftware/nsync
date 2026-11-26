# NSync

## What is NSync

NSync is another tool in the privacy-lovers toolbox. 
The vision is for NSync to be an encrypted file sync tool that works immediately with your own file storage be that Amazon S3, Cloudflare R2, Backblaze B2 or anything else you can possibly think of.

## Where it's at

NSync is at its very early stages of development. It is not stable - it's nowhere close. 

Currently NSync allows you to read S3 credentials from your environment and upload a file that's 6MiB or larger to a Cloudflare R2 bucket that YOU own. 

The code produces a blake3 hash of the file that's computed during streaming, this hash is returned to the caller and forgotten for the moment.

NSync currently has no `watch` functionality, any claim of `watch` before version 0.5.0 are **purely** vision and may or may not work.

NSync currently has no encryption, all claims of encryption before version 0.8.0 are **purely** vision and may or may not work.

## Roadmap / Todo

- Go through all TODO's in the codebase
- Manifest System (stop leaking metadata)
- Full cipher suite with AES-256-GCM-SIV

## Building & Running

`git clone [https:// | git@]github.com[: | /]NexixSoftware/nsync.git`

At this point, read through the code to fully understand what nsync is actually trying to do as there is a high chance that it will panic at runtime without fully understanding what nsync needs.

If you can't make sense of what it's trying to do or can't follow the code, wait for v0.5.

`cargo build --release` 
or
`cargo run -- --[flag]`

## IaC + Actions

Actions runners are setup with blacksmith on the main branch.

Production infrastructure will be managed in Terraform post-1.0.

## License

NSync is licensed under AGPL-3.0. Copyleft is deliberate: if you build on NSync - even as a hosted service - your improvements must be shared back.
