# Obfuscated version of Powerview

Based on:
https://raw.githubusercontent.com/PowerShellMafia/PowerSploit/master/Recon/PowerView.ps1
Downloaded 23-05-2023

## Obfuscation

- Removed Comments
- Rename functions and aliases
- Rename Variables


## Renamed functions

| Original name | Obfuscated name |
| -- | -- |
| New-InMemoryModule | New-naboo |
| Add-Win32Type | add-endor |
| psenum | exegol |
| New-DynamicParameter | new-callos |
| Get-IniContent | get-carlac  |
| Export-PowerViewCSV | Export-KaminoCSV  |
| Resolve-IPAddress | Resolve-alzar |
| ConvertTo-SID | ConvertTo-batuu |
| ConvertFrom-SID | ConvertFrom-batuu |
| Convert-ADName | Convert-crait |
| ConvertFrom-UACValue | ConvertFrom-bogden |
| Get-PrincipalContext | Get-bespin |
| Add-RemoteConnection | Add-alderaan |
| Remove-RemoteConnection | Remove-alderaan |
| Invoke-UserImpersonation | Invoke-hosnianPrime |
| Invoke-RevertToSelf | Invoke-hoth |
| Get-DomainSPNTicket | Get-geonosis |
| Invoke-Kerberoast | Invoke-mustafar |
| Get-PathAcl | Get-jedha |
| Convert-LDAPProperty | Convert-lego |
| Get-DomainSearcher | Get-jakku |
| Convert-DNSRecord | Convert-scarif |
| Get-DomainDNSZone | Get-subterrel |
| Get-DomainDNSRecord | Get-wobani |
| Get-Domain | Get-yavin |
| Get-DomainController | Get-yavinController |
| Get-Forest | Get-kessel |
| Get-ForestDomain | Get-kesselDomain |
| Get-ForestGlobalCatalog | Get-kesselGlobalCatalog |
| Get-ForestSchemaClass | Get-kesselSchemaClass |
| Find-DomainObjectPropertyOutlier | Find-takodana |
| Get-DomainUser | Get-yavinUser |
| New-DomainUser | New-yavinUser |
| Set-DomainUserPassword | Set-yavinUserPassword |
| Get-DomainUserEvent | Get-yavinUserEvent |
| Get-DomainGUIDMap | Get-yavinGUIDMap |
| Get-DomainComputer | Get-yavinComputer |
| Get-DomainObject | Get-yavinObject |
| Get-DomainObjectAttributeHistory | Get-yavinObjectAttributeHistory |
| Get-DomainObjectLinkedAttributeHistory | Get-yavinObjectLinkedAttributeHistory |
| Set-DomainObject | Set-yavinObject |
| ConvertFrom-LDAPLogonHours | ConvertFrom-utapau |
| New-ADObjectAccessControlEntry | New-draboon |
| Set-DomainObjecOwner | Set-yavinObjecOwner |
| Get-DomainObjectAcl | Get-yavinObjectAcl |
| Add-DomainObjectAcl | Add-yavinObjectAcl |
| Remove-DomainObjectAcl | Remove-yavinObjectAcl |
| Find-InterestingDomainAcl | Find-concordia |
| Get-DomainOU | Get-yavinOU |
| Get-DomainSite | Get-yavinSite |
| Get-DomainSubnet | Get-yavinSubnet |
| Get-DomainSID | Get-yavinSID |
| Get-DomainGroup | Get-yavinGroup |
| New-DomainGroup | New-yavinGroup |
| Get-DomainManagedSecurityGroup | Get-yavinManagedSecurityGroup |
| Get-DomainGroupMember | Get-yavinGroupMember |
| Get-DomainGroupMemberDeleted | Get-yavinGroupMemberDeleted |
| Add-DomainGroupMember | Add-yavinGroupMember |
| Remove-DomainGroupMember | Remove-yavinGroupMember |
| Get-DomainFileServer | Get-yavinFileServer |
| Get-DomainDFSShare | Get-yavinDFSShare |
| Get-GptTmpl | Get-Felucia |
| Get-GroupsXML | Get-mooga |
| Get-DomainGPO | Get-yavinGPO |
| Get-DomainGPOLocalGroup | Get-yavinGPOLocalGroup |
| Get-DomainGPOUserLocalGroupMapping | Get-yavinGPOUserLocalGroupMapping |
| Get-DomainGPOComputerLocalGroupMapping | Get-yavinGPOComputerLocalGroupMapping |
| Get-DomainPolicyData | Get-yavinPolicyData |
| Get-NetLocalGroup | Get-kiros |
| Get-NetLocalGroupMember | Get-kirosMember |
| Get-NetShare | Get-rodia |
| Get-NetLoggedon | Get-solarin |
| Get-NetSession | Get-tibrin |
| Get-RegLoggedOn | Get-yavikprime |
| Get-NetRDPSession | Get-quell |
| Test-AdminAccess | Test-qiilura |
| Get-NetComputerSiteName | Get-orondia |
| Get-WMIRegProxy | Get-onderon |
| Get-WMIRegLastLoggedOn | Get-mortis |
| Get-WMIRegCachedRDPConnection | Get-lothominor |
| Get-WMIRegMountedDrive | Get-kadavo |
| Get-WMIProcess | Get-mandalore |
| Find-InterestingFile | Find-coruscant |
| New-ThreadedFunction | New-abafar |
| Find-DomainUserLocation | Find-yavinUserLocation |
| Find-DomainProcess | Find-yavinProcess |
| Find-DomainUserEvent | Find-yavinUserEvent |
| Find-DomainShare | Find-yavinShare |
| Find-InterestingDomainShareFile | Find-InterestingyavinShareFile |
| Find-LocalAdminAccess | Find-Christophsis |
| Find-DomainLocalGroupMember | Find-yavinLocalGroupMember |
| Get-DomainTrust | Get-yavinTrust |
| Get-ForestTrust | Get-kesselTrust |
| Get-DomainForeignUser | Get-yavinForeignUser |
| Get-DomainForeignGroupMember | Get-yavinForeignGroupMember |
| Get-DomainTrustMapping | Get-yavinTrustMapping |
| Get-GPODelegation | Get-Dathomir |
|  |  |

Renamed Variables

| Original name | Obfuscated name |
| -- | -- |
| PowerView | Kamino |


