module
public import Init
public import SemanticFixture.KernelReductionDefinitions
set_option autoImplicit false
set_option maxRecDepth 100000
set_option maxHeartbeats 1000000000
namespace SemanticFixture.KernelReductionProofs

public theorem importedAppendLengthIsFour : (SemanticFixture.KernelReductionDefinitions.importedLength = 4) := by
  decide +kernel

end SemanticFixture.KernelReductionProofs
