resource "cloudflare_account_token" "business_account_token" {
  account_id = "d452f644c1f4b9d931d76d1d10e33089"
  name       = "nsync"

  policies = [{
    effect = "allow"
    permission_groups = [{
      id = "b4992e1108244f5d8bfbd5744320c2e1"
      }, {
      id = "bf7481a1826f439697cb59a20b22293e"
    }]
    resources = jsonencode({
      "com.cloudflare.api.account.d452f644c1f4b9d931d76d1d10e33089" = "*"
    })
  }]
}
