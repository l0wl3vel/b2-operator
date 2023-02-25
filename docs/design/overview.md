# High Level Overview

# `Account`
To do anything with the Backblaze B2 API an API key is required. A reference to this API Key is stored in the `Account` CR. Not storing the Secret in the CR itself was a conscious choice allowing for better handling of the required key material within a GitOps workflow with tools for encrypting secrets in transit (e.g. Bitnami Sealed Secrets, External Secrets). This `Account` CRD a cluster-wide resource, since it is intended to be used to derive Application `Keys` and `Buckets`, which might exist in seperate Namespaces.

A status other than "Ready" means that there is some issue with the Account. In most cases this will mean that the credentials are wrong.

# `Bucket`

All Buckets in B2 have globally unique names. Therefore, the B2 Bucket Name is equal to the `metadata.name` and also a cluster-wide resource. Bucket creation and adoption work exactly the same. Adoption is the process of making an existing B2 Bucket known to the operator.

A status other than "Ready" means that the managing `Account` does not have the required permissions to access the bucket or was created by another user.


# `Key`

A `Key` is a Key for a specific `Bucket`. It projects the B2 credentials in on (or more) Secrets. Since Keys are not globally unique they are namespaced and their `metadata.name` is not equal to the friendly name of the Key in the B2 API. They store their credentials in a secret in the same namespace, which will also be used by applications.