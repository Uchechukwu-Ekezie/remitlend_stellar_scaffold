import React, { useState } from "react";
import { useWallet } from "../hooks/useWallet";
import { CheckCircle, Clock, AlertCircle, Award } from "lucide-react";

type VerificationStep = "connect" | "processing" | "complete" | "failed";

interface VerificationData {
  provider: string;
  accountId: string;
  monthlyAmount: number;
  historyMonths: number;
  totalSent: number;
  reliabilityScore: number;
  nftTokenId?: number;
}

const VerificationFlow: React.FC = () => {
  const { connected } = useWallet();
  const [currentStep, setCurrentStep] = useState<VerificationStep>("connect");
  const [selectedProvider, setSelectedProvider] = useState("");
  const [accountId, setAccountId] = useState("");
  const [verificationData, setVerificationData] =
    useState<VerificationData | null>(null);

  const providers = [
    { id: "wise", name: "Wise (TransferWise)", logo: "💳" },
    { id: "western_union", name: "Western Union", logo: "🌍" },
    { id: "paypal", name: "PayPal", logo: "💰" },
    { id: "remitly", name: "Remitly", logo: "📱" },
  ];

  const handleSubmitVerification = () => {
    if (!selectedProvider || !accountId) return;

    setCurrentStep("processing");

    // Simulate verification process
    setTimeout(() => {
      const mockData: VerificationData = {
        provider: selectedProvider,
        accountId: accountId,
        monthlyAmount: 2500,
        historyMonths: 18,
        totalSent: 45000,
        reliabilityScore: 92,
        nftTokenId: 1,
      };

      setVerificationData(mockData);
      setCurrentStep("complete");
    }, 3000);
  };

  const getScoreColor = (score: number) => {
    if (score >= 90) return "#10b981";
    if (score >= 80) return "#fbbf24";
    if (score >= 70) return "#fb923c";
    return "#ef4444";
  };

  const getScoreMeaning = (score: number) => {
    if (score >= 90) return "Excellent - You qualify for the best loan terms!";
    if (score >= 80) return "Very Good - You qualify for competitive rates.";
    if (score >= 70) return "Good - You can access loans with standard terms.";
    return "Fair - Limited loan options available.";
  };

  if (!connected) {
    return (
      <div className="min-h-screen relative overflow-hidden flex items-center justify-center p-8">
        <div className="glass rounded-2xl shadow-strong p-12 text-center max-w-md border border-gray-300 dark:border-white/10 backdrop-blur-xl">
          <AlertCircle className="w-16 h-16 text-warning-500 mx-auto mb-6" />
          <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-4">
            Please Connect Your Wallet
          </h2>
          <p className="text-gray-700 dark:text-white text-lg">
            You need to connect your wallet to start the verification process
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen relative overflow-hidden">
      {/* Animated Background Blobs */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none mx-auto">
        <div className="absolute top-0 left-1/4 w-96 h-96 bg-indigo-500/10 rounded-full blur-3xl animate-float"></div>
        <div
          className="absolute top-1/4 right-1/4 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-float"
          style={{ animationDelay: "2s" }}
        ></div>
        <div
          className="absolute bottom-1/4 left-1/3 w-96 h-96 bg-cyan-500/10 rounded-full blur-3xl animate-float"
          style={{ animationDelay: "4s" }}
        ></div>
      </div>

      <div className="relative max-w-4xl mx-auto p-8">
        <div className="text-center mb-12">
          <h1 className="text-5xl font-bold text-gradient-cyber mb-4">
            Remittance Verification
          </h1>
          <p className="text-xl text-gray-700 dark:text-white">
            Verify your remittance history to create your NFT and unlock lending
          </p>
        </div>

        {/* Progress Steps */}
        <div className="flex items-center justify-center mb-12">
          <div
            className={`flex flex-col items-center ${currentStep === "connect" ? "opacity-100" : ["processing", "complete"].includes(currentStep) ? "opacity-100" : "opacity-50"}`}
          >
            <div
              className={`w-16 h-16 rounded-full flex items-center justify-center text-xl font-bold transition-all duration-300 ${
                ["processing", "complete"].includes(currentStep)
                  ? "bg-emerald-500 text-white shadow-lg shadow-emerald-500/50"
                  : currentStep === "connect"
                    ? "bg-indigo-500 text-white shadow-lg shadow-indigo-500/50"
                    : "bg-white/10 text-slate-400"
              }`}
            >
              {["processing", "complete"].includes(currentStep) ? (
                <CheckCircle className="w-8 h-8" />
              ) : (
                "1"
              )}
            </div>
            <p className="mt-2 font-semibold text-gray-700 dark:text-white">
              Connect Provider
            </p>
          </div>

          <div className="w-24 h-1 mx-4 bg-gray-300 dark:bg-white/10 rounded-full overflow-hidden">
            <div
              className={`h-full bg-indigo-500 transition-all duration-500 rounded-full ${["processing", "complete"].includes(currentStep) ? "w-full" : "w-0"}`}
            ></div>
          </div>

          <div
            className={`flex flex-col items-center ${currentStep === "processing" ? "opacity-100" : currentStep === "complete" ? "opacity-100" : "opacity-50"}`}
          >
            <div
              className={`w-16 h-16 rounded-full flex items-center justify-center text-xl font-bold transition-all duration-300 ${
                currentStep === "complete"
                  ? "bg-emerald-500 text-white shadow-lg shadow-emerald-500/50"
                  : currentStep === "processing"
                    ? "bg-indigo-500 text-white shadow-lg shadow-indigo-500/50 animate-pulse"
                    : "bg-white/10 text-slate-400"
              }`}
            >
              {currentStep === "complete" ? (
                <CheckCircle className="w-8 h-8" />
              ) : (
                "2"
              )}
            </div>
            <p className="mt-2 font-semibold text-gray-700 dark:text-white">
              Verification
            </p>
          </div>

          <div className="w-24 h-1 mx-4 bg-gray-300 dark:bg-white/10 rounded-full overflow-hidden">
            <div
              className={`h-full bg-indigo-500 transition-all duration-500 rounded-full ${currentStep === "complete" ? "w-full" : "w-0"}`}
            ></div>
          </div>

          <div
            className={`flex flex-col items-center ${currentStep === "complete" ? "opacity-100" : "opacity-50"}`}
          >
            <div
              className={`w-16 h-16 rounded-full flex items-center justify-center text-xl font-bold transition-all duration-300 ${
                currentStep === "complete"
                  ? "bg-emerald-500 text-white shadow-lg shadow-emerald-500/50"
                  : "bg-white/10 text-slate-400"
              }`}
            >
              3
            </div>
            <p className="mt-2 font-semibold text-gray-700 dark:text-white">
              NFT Minted
            </p>
          </div>
        </div>

        {/* Connect Provider Step */}
        {currentStep === "connect" && (
          <div className="max-w-2xl mx-auto">
            <div className="glass rounded-2xl shadow-strong p-8 border border-gray-300 dark:border-white/10 backdrop-blur-xl card-shine">
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-4">
                Select Your Remittance Provider
              </h2>
              <p className="text-gray-700 dark:text-white text-lg mb-8">
                Choose the service you use to send money internationally. We'll
                verify your payment history securely.
              </p>

              <div className="grid grid-cols-2 gap-4 mb-8">
                {providers.map((provider) => (
                  <button
                    key={provider.id}
                    type="button"
                    className={`p-6 rounded-xl border-2 transition-all duration-200 flex flex-col items-center gap-3 hover:shadow-medium hover:scale-105 ${
                      selectedProvider === provider.id
                        ? "border-indigo-500 bg-indigo-500/20 shadow-lg shadow-indigo-500/30"
                        : "border-gray-300 dark:border-white/20 glass bg-white/5 dark:bg-white/5 hover:border-indigo-500/50"
                    }`}
                    onClick={() => setSelectedProvider(provider.id)}
                  >
                    <span className="text-4xl">{provider.logo}</span>
                    <span className="font-semibold text-gray-900 dark:text-white">
                      {provider.name}
                    </span>
                  </button>
                ))}
              </div>

              {selectedProvider && (
                <div className="mb-8 animate-fade-in">
                  <label className="block text-sm font-semibold text-gray-700 dark:text-white mb-2">
                    Account ID / Email
                  </label>
                  <input
                    type="text"
                    value={accountId}
                    onChange={(e) => setAccountId(e.target.value)}
                    placeholder="Enter your account identifier"
                    className="w-full text-lg p-4 glass bg-white/5 dark:bg-white/5 border-2 border-gray-300 dark:border-white/20 rounded-xl focus:border-indigo-500 focus:ring-4 focus:ring-indigo-500/20 outline-none transition-all text-gray-900 dark:text-white placeholder:text-gray-500 dark:placeholder:text-slate-500"
                  />
                  <p className="text-sm text-gray-700 dark:text-white mt-2">
                    We'll use this to securely fetch your remittance history via
                    our oracle.
                  </p>
                </div>
              )}

              <button
                type="button"
                className={`w-full py-4 px-6 rounded-xl font-bold text-white text-lg transition-all duration-200 ${
                  !selectedProvider || !accountId
                    ? "bg-gray-200 dark:bg-white/10 cursor-not-allowed text-gray-500 dark:text-white"
                    : "bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 shadow-lg shadow-indigo-500/30 hover:shadow-indigo-500/50 transform hover:scale-105"
                }`}
                onClick={handleSubmitVerification}
                disabled={!selectedProvider || !accountId}
              >
                Start Verification
              </button>

              <div className="mt-6 glass bg-indigo-500/10 border-l-4 border-indigo-500 rounded-xl p-4">
                <p className="text-sm text-indigo-300">
                  🔒 Your data is encrypted and processed securely by our oracle
                  network
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Processing Step */}
        {currentStep === "processing" && (
          <div className="max-w-2xl mx-auto">
            <div className="glass rounded-2xl shadow-strong p-8 text-center border border-gray-300 dark:border-white/10 backdrop-blur-xl card-shine">
              <div className="mb-6">
                <Clock
                  className="w-20 h-20 text-indigo-400 mx-auto animate-spin"
                  style={{ animationDuration: "3s" }}
                />
              </div>
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white mb-4">
                Verifying Your Remittance History
              </h2>
              <p className="text-gray-700 dark:text-white text-lg mb-8">
                Our oracle is securely connecting to{" "}
                {providers.find((p) => p.id === selectedProvider)?.name}
                to verify your payment history. This usually takes 1-3 minutes.
              </p>

              <div className="space-y-4 mb-8">
                <div className="flex items-center gap-3 p-4 glass bg-emerald-500/10 rounded-lg border border-emerald-500/20">
                  <CheckCircle className="w-6 h-6 text-emerald-400 flex-shrink-0" />
                  <span className="text-gray-800 dark:text-white font-medium">
                    Connected to{" "}
                    {providers.find((p) => p.id === selectedProvider)?.name}
                  </span>
                </div>
                <div className="flex items-center gap-3 p-4 glass bg-warning-500/10 rounded-lg border border-warning-500/20">
                  <Clock className="w-6 h-6 text-warning-400 flex-shrink-0 animate-pulse" />
                  <span className="text-gray-800 dark:text-white font-medium">
                    Fetching payment history...
                  </span>
                </div>
                <div className="flex items-center gap-3 p-4 glass bg-warning-500/10 rounded-lg border border-warning-500/20">
                  <Clock className="w-6 h-6 text-warning-400 flex-shrink-0 animate-pulse" />
                  <span className="text-gray-800 dark:text-white font-medium">
                    Calculating reliability score...
                  </span>
                </div>
              </div>

              <div className="glass bg-white/5 dark:bg-white/5 rounded-xl p-4 border border-gray-300 dark:border-white/10">
                <p className="text-gray-700 dark:text-white">
                  Estimated time remaining:{" "}
                  <strong className="text-gray-900 dark:text-white">
                    2 minutes
                  </strong>
                </p>
              </div>
            </div>
          </div>
        )}

        {/* Completion Step */}
        {currentStep === "complete" && verificationData && (
          <div className="max-w-3xl mx-auto">
            <div className="glass rounded-2xl shadow-strong p-8 border border-gray-300 dark:border-white/10 backdrop-blur-xl card-shine">
              <div className="text-center mb-8">
                <CheckCircle className="w-20 h-20 text-emerald-400 mx-auto mb-4" />
                <h2 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
                  Verification Complete! 🎉
                </h2>
                <p className="text-gray-700 dark:text-white text-lg">
                  Your Remittance NFT has been successfully minted to your
                  wallet.
                </p>
              </div>

              <div className="space-y-8">
                <div className="glass bg-gradient-to-br from-indigo-500/10 to-purple-500/10 rounded-2xl p-8 border-2 border-indigo-500/20">
                  <div className="flex items-center gap-3 mb-6">
                    <Award className="w-10 h-10 text-indigo-400" />
                    <h3 className="text-2xl font-bold text-gray-900 dark:text-white">
                      Remittance NFT #{verificationData.nftTokenId}
                    </h3>
                  </div>

                  <div className="flex flex-col items-center mb-6">
                    <div
                      className="w-40 h-40 rounded-full border-8 flex flex-col items-center justify-center glass bg-white/5 shadow-lg mb-4"
                      style={{
                        borderColor: getScoreColor(
                          verificationData.reliabilityScore,
                        ),
                      }}
                    >
                      <div
                        className="text-5xl font-bold"
                        style={{
                          color: getScoreColor(
                            verificationData.reliabilityScore,
                          ),
                        }}
                      >
                        {verificationData.reliabilityScore}
                      </div>
                      <div className="text-sm text-gray-700 dark:text-white font-semibold mt-2">
                        Reliability Score
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-3 gap-4">
                    <div className="glass bg-white/5 dark:bg-white/5 rounded-lg p-4 text-center border border-gray-300 dark:border-white/10">
                      <span className="text-sm text-gray-700 dark:text-white block mb-1">
                        Monthly Remittance
                      </span>
                      <span className="text-xl font-bold text-gray-900 dark:text-white">
                        ${verificationData.monthlyAmount.toLocaleString()}
                      </span>
                    </div>
                    <div className="glass bg-white/5 dark:bg-white/5 rounded-lg p-4 text-center border border-gray-300 dark:border-white/10">
                      <span className="text-sm text-gray-700 dark:text-white block mb-1">
                        History Length
                      </span>
                      <span className="text-xl font-bold text-gray-900 dark:text-white">
                        {verificationData.historyMonths} months
                      </span>
                    </div>
                    <div className="glass bg-white/5 dark:bg-white/5 rounded-lg p-4 text-center border border-gray-300 dark:border-white/10">
                      <span className="text-sm text-gray-700 dark:text-white block mb-1">
                        Total Sent
                      </span>
                      <span className="text-xl font-bold text-gray-900 dark:text-white">
                        ${verificationData.totalSent.toLocaleString()}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="glass bg-white/5 dark:bg-white/5 rounded-xl p-6 border border-gray-300 dark:border-white/10">
                  <h4 className="text-xl font-bold text-gray-900 dark:text-white mb-3">
                    What does this mean?
                  </h4>
                  <p
                    className="text-lg font-semibold mb-3"
                    style={{
                      color: getScoreColor(verificationData.reliabilityScore),
                    }}
                  >
                    {getScoreMeaning(verificationData.reliabilityScore)}
                  </p>
                  <p className="text-gray-700 dark:text-white">
                    Your reliability score is based on your payment consistency
                    over the last 24 months. Continue making regular remittances
                    to improve your score and get better loan terms.
                  </p>
                </div>

                <div className="glass bg-indigo-500/10 rounded-xl p-6 border-l-4 border-indigo-500">
                  <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-3">
                    Next Steps
                  </h3>
                  <ul className="space-y-2 text-gray-700 dark:text-white">
                    <li className="flex items-start gap-2">
                      <span className="text-indigo-400">✓</span>
                      <span>Your NFT is now stored in your wallet</span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-indigo-400">✓</span>
                      <span>
                        You can use it as collateral to apply for loans
                      </span>
                    </li>
                    <li className="flex items-start gap-2">
                      <span className="text-indigo-400">✓</span>
                      <span>
                        Your score updates automatically as you send more
                        remittances
                      </span>
                    </li>
                  </ul>
                </div>

                <button
                  type="button"
                  className="w-full bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold py-4 px-6 rounded-xl transition-all duration-200 shadow-lg shadow-indigo-500/30 hover:shadow-indigo-500/50 transform hover:scale-105 text-lg"
                  onClick={() => {
                    window.location.href = "/borrow";
                  }}
                >
                  Apply for a Loan Now
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default VerificationFlow;
