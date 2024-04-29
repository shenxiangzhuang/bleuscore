import json
import matplotlib.pyplot as plt


if __name__ == "__main__":

    # Load the data
    with open('result.json') as f:
        data = json.load(f)

    # Extract the mean times for each N
    N_values = [1000, 10000, 100000, 1000000]
    local_hf_bleu_times = []
    rs_bleuscore_times = []
    for result in data['results']:
        N = int(result['command'].split()[-1])
        if N in N_values:
            if 'local_hf_bleu' in result['command']:
                local_hf_bleu_times.append((N, result['mean']))
            elif 'rs_bleuscore' in result['command']:
                rs_bleuscore_times.append((N, result['mean']))
    # Calculate the speedup multiples
    speed_up_multiples = []
    for i in range(len(N_values)):
        local_hf_bleu_time = local_hf_bleu_times[i][1]
        rs_bleuscore_time = rs_bleuscore_times[i][1]
        speed_up_multiple = local_hf_bleu_time / rs_bleuscore_time
        speed_up_multiples.append(speed_up_multiple)

    # Plot the bar plot
    plt.bar(N_values, [t[1] for t in local_hf_bleu_times], label='local_hf_bleu', width=N_values, color='#3498db')
    plt.bar(N_values, [t[1] for t in rs_bleuscore_times], label='rs_bleuscore', width=N_values, color='#f1c40f')
    plt.xscale('log')  # log scale for x-axis
    plt.xlabel('Corpus Size (N)', fontsize=14)
    plt.ylabel('Mean Time (Second)', fontsize=14)
    plt.title('Performance Comparison', fontsize=14)
    plt.legend(fontsize=14)
    plt.tight_layout()

    # Add the speedup multiples as annotations
    for i, N in enumerate(N_values):
        plt.annotate(f'x{speed_up_multiples[i]:.2f}', (N, local_hf_bleu_times[i][1]),
                     ha='center', va='bottom', fontsize=12)

    plt.savefig("bench.png", dpi=300)
